
use jni_sys::JNINativeInterface_;

pub fn main() {
    unsafe {
        let jni = std::mem::zeroed::<JNINativeInterface_>();
        let _1_2_function = jni.v1_1.FromReflectedMethod;
    }
}
