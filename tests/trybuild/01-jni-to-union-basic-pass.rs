use jni_sys::{jint, JNIEnv};
use jni_sys_macros::jni_to_union;
use std::os::raw::c_void;

#[jni_to_union]
pub struct MyStruct {
    #[jni_added("reserved")]
    pub reserved0: *mut c_void,

    #[jni_added("1.1")]
    pub GetVersion: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

    pub FunctionA: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,

    #[jni_added("1.2")]
    pub FunctionB: unsafe extern "system" fn(env: *mut JNIEnv) -> jint,
}

pub fn main() {
    assert_eq!(std::mem::size_of::<MyStruct>(), std::mem::size_of::<*mut c_void>() * 4);
}
