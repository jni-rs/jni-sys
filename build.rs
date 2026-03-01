fn main() {
    if cfg!(feature = "jvm_examples") {
        let java_home = std::env::var("JAVA_HOME").unwrap();
        println!("cargo:rustc-link-search=native={}/lib/server", java_home);
        println!("cargo:rustc-link-search=native={}/lib", java_home);
    }
}
