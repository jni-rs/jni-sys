use jni_sys::{jint, JNIEnv, JNINativeInterface_};
use jni_sys_macros::jni_to_union;
use std::os::raw::c_void;

#[test]
fn jni_to_union_trybuilds() {
    let t = trybuild::TestCases::new();
    t.pass("tests/trybuild/01-jni-to-union-basic-pass.rs");
    t.compile_fail("tests/trybuild/01-jni-fail-reserved-read.rs");
    t.compile_fail("tests/trybuild/01-jni-fail-read-1-2-from-1-1.rs");
}

#[test]
fn jni_to_union() {
    #[repr(C)]
    #[jni_to_union]
    pub struct MyStruct {
        #[jni_added("reserved")]
        pub reserved0: *mut c_void,

        #[jni_added("1.1")]
        pub GetVersion: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

        #[jni_added("1.2")]
        pub FunctionA: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

        pub FunctionB: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

        #[jni_added("1.3")]
        pub FunctionC: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,
    }

    assert_eq!(
        std::mem::size_of::<MyStruct>(),
        std::mem::size_of::<*mut c_void>() * 5
    );
    assert_eq!(
        std::mem::size_of::<MyStruct_1_1>(),
        std::mem::size_of::<*mut c_void>() * 4
    );
}

const NUM_JNI_ENV_MEMBERS: usize = 236;
#[test]
fn jni_env_union() {
    assert_eq!(
        std::mem::size_of::<JNINativeInterface_>(),
        std::mem::size_of::<*mut c_void>() * NUM_JNI_ENV_MEMBERS
    );
}
