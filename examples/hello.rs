#![cfg(feature = "jvm_examples")]
//! This minimal example shows how to use the JNI API to create a JavaVM, then
//! attach the current thread to the JVM, instantiate an object and then call a
//! method on that object.
//!
//! This requires the "jvm_examples" feature so that `build.rs` will add
//! `$JAVA_HOME/lib` and `$JAVA_HOME/lib/server` to the shared library search
//! path so that the example can link directly with `libjvm`

use std::{ffi::c_void, ptr};

use jni_sys as sys;
use thiserror::Error;

macro_rules! java_vm_call_unchecked {
    ( $jvm:expr, $version:tt, $name:ident $(, $args:expr )*) => {{
        let jvm: *mut jni_sys::JavaVM = $jvm;
        ((*(*jvm)).$version.$name)(jvm $(, $args)*)
    }};
}

macro_rules! jni_call_unchecked {
    ( $jnienv:expr, $version:tt, $name:ident $(, $args:expr )*) => {{
        let env: *mut jni_sys::JNIEnv = $jnienv;
        let interface: *const jni_sys::JNINativeInterface_ = *env;
        ((*interface).$version.$name)(env $(, $args)*)
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

unsafe fn attach_current_thread(vm: *mut sys::JavaVM) -> Result<*mut sys::JNIEnv> {
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

#[link(name = "jvm")]
unsafe extern "system" {
    pub fn JNI_CreateJavaVM(
        pvm: *mut *mut sys::JavaVM,
        penv: *mut *mut c_void,
        args: *mut c_void,
    ) -> sys::jint;
}

fn main() -> Result<()> {
    unsafe {
        let opts: Vec<sys::JavaVMOption> = vec![sys::JavaVMOption {
            optionString: c"-Xcheck:jni".as_ptr() as _,
            extraInfo: ptr::null_mut(),
        }];
        let args = sys::JavaVMInitArgs {
            version: sys::JNI_VERSION_1_4,
            ignoreUnrecognized: false,
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

        let env = attach_current_thread(jvm)?;

        // Find the java.lang.Integer class
        let integer_class =
            jni_call_unchecked!(env, v1_1, FindClass, c"java/lang/Integer".as_ptr());
        if integer_class.is_null() {
            eprintln!("Failed to find Integer class");
            return Err(JniError::Unknown);
        }

        // Get the constructor method ID for Integer(int)
        let ctor_id = jni_call_unchecked!(
            env,
            v1_1,
            GetMethodID,
            integer_class,
            c"<init>".as_ptr(),
            c"(I)V".as_ptr()
        );
        if ctor_id.is_null() {
            eprintln!("Failed to get Integer constructor");
            return Err(JniError::Unknown);
        }

        // Create an Integer object with value 42 using NewObjectA
        let args = [sys::jvalue { i: 42 }];
        let integer_obj =
            jni_call_unchecked!(env, v1_1, NewObjectA, integer_class, ctor_id, args.as_ptr());
        if integer_obj.is_null() {
            eprintln!("Failed to create Integer object");
            return Err(JniError::Unknown);
        }

        println!("Created Integer object with value 42");

        // Get the intValue() method ID
        let intvalue_method = jni_call_unchecked!(
            env,
            v1_1,
            GetMethodID,
            integer_class,
            c"intValue".as_ptr(),
            c"()I".as_ptr()
        );
        if intvalue_method.is_null() {
            eprintln!("Failed to get intValue method");
            return Err(JniError::Unknown);
        }

        // Call the intValue() method
        let value = jni_call_unchecked!(
            env,
            v1_1,
            CallIntMethodA,
            integer_obj,
            intvalue_method,
            ptr::null()
        );
        if jni_call_unchecked!(env, v1_2, ExceptionCheck) != sys::JNI_FALSE {
            jni_call_unchecked!(env, v1_1, ExceptionDescribe);
            jni_call_unchecked!(env, v1_1, ExceptionClear);
            return Err(JniError::Unknown);
        }
        println!("Called intValue() and got: {}", value);

        // Detach the current thread
        let detach_ret = java_vm_call_unchecked!(jvm, v1_1, DetachCurrentThread);
        jni_error_code_to_result(detach_ret)?;

        // Destroy the JVM
        let destroy_ret = java_vm_call_unchecked!(jvm, v1_1, DestroyJavaVM);
        jni_error_code_to_result(destroy_ret)?;

        println!("JVM destroyed successfully");
    }

    Ok(())
}
