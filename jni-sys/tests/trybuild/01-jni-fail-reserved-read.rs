
use jni_sys::JNINativeInterface_;

pub fn main() {
    unsafe {
        let jni = std::mem::zeroed::<JNINativeInterface_>();
        let _reserved = jni.v1_1.reserved0;
    }
}
