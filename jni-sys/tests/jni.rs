#![cfg(jvm_linked)]

//! Make sure to set JAVA_HOME and _JNI_SYS_TEST=1 in the environment when
//! running `cargo test` to link against libjvm.

use std::ptr;

use jni_sys as sys;

#[path = "utils.rs"]
mod utils;

use utils::*;

#[test]
fn invocation_jni_test() {
    unsafe {
        let jvm = get_java_vm().unwrap();

        let env = attach_current_thread(jvm).unwrap();

        // Find the java.lang.Integer class
        let integer_class =
            jni_call_unchecked!(env, v1_1, FindClass, c"java/lang/Integer".as_ptr());
        assert!(!integer_class.is_null(), "Failed to find Integer class");

        // Get the constructor method ID for Integer(int)
        let ctor_id = jni_call_unchecked!(
            env,
            v1_1,
            GetMethodID,
            integer_class,
            c"<init>".as_ptr(),
            c"(I)V".as_ptr()
        );
        assert!(!ctor_id.is_null(), "Failed to get Integer constructor");

        // Create an Integer object with value 42 using NewObjectA
        let args = [sys::jvalue { i: 42 }];
        let integer_obj =
            jni_call_unchecked!(env, v1_1, NewObjectA, integer_class, ctor_id, args.as_ptr());
        assert!(!integer_obj.is_null(), "Failed to create Integer object");

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
        assert!(!intvalue_method.is_null(), "Failed to get intValue method");

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
            panic!("JNI exception occurred");
        }
        println!("Called intValue() and got: {}", value);

        detach_current_thread(jvm).unwrap();

        // Destroy the JVM
        let destroy_ret = java_vm_call_unchecked!(jvm, v1_1, DestroyJavaVM);
        jni_error_code_to_result(destroy_ret).unwrap();

        println!("JVM destroyed successfully");
    }
}
