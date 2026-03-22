#![allow(unused)]

use std::{ffi::c_void, ptr};

use jni_sys as sys;
use thiserror::Error;

#[macro_export]
macro_rules! java_vm_call_unchecked {
    ( $jvm:expr, $version:tt, $name:ident $(, $args:expr )*) => {{
        let jvm: *mut jni_sys::JavaVM = $jvm;
        ((*(*jvm)).$name).unwrap()(jvm $(, $args)*)
    }};
}

#[macro_export]
macro_rules! jni_call_unchecked {
    ( $jnienv:expr, $version:tt, $name:ident $(, $args:expr )*) => {{
        let env: *mut jni_sys::JNIEnv = $jnienv;
        let interface: *const jni_sys::JNINativeInterface_ = *env;
        ((*interface).$name).unwrap()(env $(, $args)*)
    }};
}

#[derive(Debug, Error)]
pub enum JniError {
    #[error("Unknown error")]
    Unknown,
    #[error("Current thread is not attached to the Java VM")]
    ThreadDetached,
    #[error("JNI version error")]
    WrongVersion,
    #[error("Not enough memory")]
    NoMemory,
    #[error("VM already created")]
    AlreadyCreated,
    #[error("Invalid arguments")]
    InvalidArguments,
    #[error("Error code {0}")]
    Other(sys::jint),
}

type Result<T> = std::result::Result<T, JniError>;

pub fn jni_error_code_to_result(code: sys::jint) -> Result<()> {
    match code {
        sys::JNI_OK => Ok(()),
        sys::JNI_ERR => Err(JniError::Unknown),
        sys::JNI_EDETACHED => Err(JniError::ThreadDetached),
        sys::JNI_EVERSION => Err(JniError::WrongVersion),
        sys::JNI_ENOMEM => Err(JniError::NoMemory),
        sys::JNI_EEXIST => Err(JniError::AlreadyCreated),
        sys::JNI_EINVAL => Err(JniError::InvalidArguments),
        _ => Err(JniError::Other(code)),
    }
}

struct JavaVMWrapper {
    jvm: *mut sys::JavaVM,
}
unsafe impl Send for JavaVMWrapper {}
unsafe impl Sync for JavaVMWrapper {}
impl JavaVMWrapper {
    fn new(jvm: *mut sys::JavaVM) -> Self {
        Self { jvm }
    }
    fn ptr(&self) -> *mut sys::JavaVM {
        self.jvm
    }
}
static SINGLETON_JVM: std::sync::OnceLock<JavaVMWrapper> = std::sync::OnceLock::new();

pub fn get_java_vm() -> Result<*mut sys::JavaVM> {
    let jvm = SINGLETON_JVM.get_or_init(|| {
        let jvm = create_java_vm().expect("Failed to create Java VM");
        JavaVMWrapper::new(jvm)
    });
    Ok(jvm.ptr())
}

/// Creates a Java VM and returns a pointer to it.
pub fn create_java_vm() -> Result<*mut sys::JavaVM> {
    unsafe {
        let opts: Vec<sys::JavaVMOption> = vec![sys::JavaVMOption {
            optionString: c"-Xcheck:jni".as_ptr() as _,
            extraInfo: ptr::null_mut(),
        }];
        let args = sys::JavaVMInitArgs {
            version: sys::JNI_VERSION_1_4,
            ignoreUnrecognized: jni_sys::JNI_FALSE,
            options: opts.as_ptr() as _,
            nOptions: opts.len() as _,
        };

        let mut jvm: *mut sys::JavaVM = ::std::ptr::null_mut();
        let mut env: *mut sys::JNIEnv = ::std::ptr::null_mut();
        let ret = JNI_CreateJavaVM(
            &mut jvm as *mut _,
            &mut env as *mut *mut sys::JNIEnv as *mut *mut c_void,
            &args as *const _ as _,
        );
        jni_error_code_to_result(ret)?;
        Ok(jvm)
    }
}

/// Attaches the current thread to the given Java VM and returns the JNI environment pointer.
///
/// # Safety
///
/// The caller must ensure that `vm` is a valid pointer to a JavaVM
pub unsafe fn attach_current_thread(vm: *mut sys::JavaVM) -> Result<*mut sys::JNIEnv> {
    let mut env_ptr = ptr::null_mut();

    let mut args = sys::JavaVMAttachArgs {
        version: sys::JNI_VERSION_1_4,
        name: ptr::null_mut(),
        group: ptr::null_mut(),
    };
    let res = unsafe {
        java_vm_call_unchecked!(
            vm,
            v1_1,
            AttachCurrentThread,
            &mut env_ptr,
            &mut args as *mut jni_sys::JavaVMAttachArgs as *mut core::ffi::c_void
        )
    };
    jni_error_code_to_result(res)?;

    Ok(env_ptr as *mut jni_sys::JNIEnv)
}

/// Detaches the current thread from the given Java VM.
///
/// # Safety
///
/// The caller must ensure that `vm` is a valid pointer to a JavaVM
pub unsafe fn detach_current_thread(vm: *mut sys::JavaVM) -> Result<()> {
    let res = unsafe { java_vm_call_unchecked!(vm, v1_1, DetachCurrentThread) };
    jni_error_code_to_result(res)
}

#[link(name = "jvm")]
extern "system" {
    pub fn JNI_CreateJavaVM(
        pvm: *mut *mut sys::JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> sys::jint;
}
