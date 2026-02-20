use std::env;
use std::path::PathBuf;

use ctest2::TestGenerator;

trait GenTest {
    fn export(&mut self, krate: &str);
}

impl GenTest for TestGenerator {
    fn export(&mut self, krate: &str) {
        self.header(&format!("{krate}.h"))
            .generate(format!("../{krate}-sys/src/lib.rs"), &format!("{krate}.rs"));
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Target {
    Linux,
    Windows,
    Darwin,
}

impl Target {
    fn new() -> Self {
        let target = env::var("TARGET").expect("'TARGET' variable set");
        if target.contains("linux") {
            Self::Linux
        } else if target.contains("windows") {
            Self::Windows
        } else if target.contains("darwin") {
            Self::Darwin
        } else {
            panic!("unsupported target");
        }
    }

    fn platform_dir(&self) -> &'static str {
        match self {
            Target::Linux => "linux",
            Target::Windows => "windows",
            Target::Darwin => "darwin",
        }
    }

    fn lib_dir(&self) -> &'static str {
        match self {
            Target::Linux | Target::Darwin => "lib/server",
            Target::Windows => "lib",
        }
    }
}

struct Config {
    java_home: PathBuf,
    target: Target,
}

impl Config {
    fn new() -> Self {
        let target = Target::new();
        let java_home = env::var_os("JAVA_HOME")
            .map(PathBuf::from)
            .expect("'JAVA_HOME' variable set");

        let native = java_home.join(target.lib_dir());
        println!("cargo:rerun-if-env-changed=JAVA_HOME");
        println!("cargo:rustc-link-search=native={}", native.display());
        println!("cargo:rustc-link-lib=dylib=jvm");
        // Increase the stack size on Windows otherwise the tests just overflow
        // the stack.
        if env::var("CARGO_CFG_TARGET_ENV").unwrap() == "msvc" {
            println!("cargo:rustc-link-arg=/stack:{}", 8 * 1024 * 1024);
        }

        Self { java_home, target }
    }

    fn test(&self) -> TestGenerator {
        let target = self.target;
        let mut test = TestGenerator::new();
        let mut includes = self.java_home.join("include");
        test.include(&includes);
        includes.push(self.target.platform_dir());
        test.include(&includes);
        test.skip_fn_ptrcheck(move |_| target == Target::Windows); // dllimport weirdness?
        test.type_name(|s, is_struct, _is_union| {
            if is_struct && (s.starts_with('_') || s.ends_with('_')) {
                format!("struct {}", s)
            } else {
                s.to_string()
            }
        });
        test
    }
}

fn main() {
    let cfg = Config::new();
    cfg.test()
        .skip_field(|s, _| matches!(s, "JNINativeInterface_" | "JNIInvokeInterface_")) // ctest2 isn't able to test these unions
        .skip_type(|s| s == "va_list")
        .skip_const(|konst| match konst {
            "JNI_VERSION_19" => !cfg!(feature = "jni19"),
            "JNI_VERSION_20" => !cfg!(feature = "jni20"),
            "JNI_VERSION_21" => !cfg!(feature = "jni21"),
            "JNI_VERSION_24" => !cfg!(feature = "jni24"),
            _ => false,
        })
        .skip_signededness(|s| {
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
        })
        .skip_roundtrip(|s| {
            matches!(
                s,
                "JNINativeInterface_" | "JNIInvokeInterface_" | "jboolean"
            )
        }) // We don't need to be able to roundtrip all possible u8 values for a jboolean, since only 0 are 1 are considered valid.
        .export("jni");
}
