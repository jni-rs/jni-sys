use std::env;
use std::path::PathBuf;

fn main() {
    let java_home = PathBuf::from(env::var_os("JAVA_HOME").unwrap());
    println!("cargo:rerun-if-env-changed=JAVA_HOME");
    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    let (platform_dir, lib_dir) = if target.contains("linux") {
        ("linux", "lib/server")
    } else if target.contains("windows") {
        ("win32", "lib")
    } else if target.contains("darwin") {
        ("darwin", "lib/server")
    } else {
        panic!("unsupported target");
    };

    println!(
        "cargo:rustc-link-search=native={}",
        java_home.join(lib_dir).display()
    );
    println!("cargo:rustc-link-lib=dylib=jvm");

    // Increase the stack size on Windows otherwise the tests just overflow
    // the stack.
    if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
        println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
    }

    let mut cfg = ctest2::TestGenerator::new();

    let include_dir = java_home.join("include");
    cfg.include(&include_dir)
        .include(include_dir.join(platform_dir));

    cfg.skip_const(|s| {
        (!cfg!(feature = "jni19") && s == "JNI_VERSION_19")
            || (!cfg!(feature = "jni20") && s == "JNI_VERSION_20")
            || (!cfg!(feature = "jni21") && s == "JNI_VERSION_21")
            || (!cfg!(feature = "jni24") && s == "JNI_VERSION_24")
    });
    cfg.skip_type(|s| s == "va_list");
    cfg.skip_field(|s, field| {
        (s == "jvalue" && field == "_data")
            || s == "JNINativeInterface_"
            || s == "JNIInvokeInterface_" // ctest2 isn't able to test these unions
    });
    cfg.type_name(|s, is_struct, _is_union| {
        if is_struct && s.ends_with('_') {
            format!("struct {}", s)
        } else {
            s.to_string()
        }
    });
    cfg.skip_signededness(|s| {
        matches!(
            s,
            "jfloat"
                | "jdouble"
                | "jobject"
                | "jclass"
                | "jstring"
                | "jarray"
                | "jbooleanArray"
                | "jbyteArray"
                | "jcharArray"
                | "jshortArray"
                | "jintArray"
                | "jlongArray"
                | "jfloatArray"
                | "jdoubleArray"
                | "jobjectArray"
                | "jweak"
                | "jthrowable"
                | "jfieldID"
                | "jmethodID"
                | "JNIEnv"
                | "JavaVM"
        )
    });
    cfg.skip_fn_ptrcheck(move |_name| {
        // dllimport weirdness?
        windows
    });
    cfg.skip_roundtrip(|s| {
        s == "jboolean" || // We don't need to be able to roundtrip all possible u8 values for a jboolean, since only 0 are 1 are considered valid.
        s == "JNINativeInterface_" || s == "JNIInvokeInterface_" // ctest2 isn't able to test these unions
    });
    cfg.header("jni.h")
        .generate("../jni-sys/src/lib.rs", "all.rs");
}
