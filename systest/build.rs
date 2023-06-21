use std::env;
use std::path::PathBuf;

fn main() {
    let java_home = PathBuf::from(env::var_os("JAVA_HOME").unwrap());
    let target = env::var("TARGET").unwrap();
    let windows = target.contains("windows");

    let (platform_dir, lib_dir) = if target.contains("linux") {
        ("linux", "jre/lib/amd64/server")
    } else if target.contains("windows") {
        ("win32", "lib")
    } else if target.contains("darwin") {
        ("darwin", "jre/lib/server")
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

    cfg.skip_type(|s| s == "va_list");
    cfg.skip_field(|s, field| s == "jvalue" && field == "_data");
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
    cfg.header("jni.h").generate("../src/lib.rs", "all.rs");
}
