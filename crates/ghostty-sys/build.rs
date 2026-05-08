fn main() {
    let lib_dir = std::env::var("DEP_GHOSTTY_LIB_DIR")
        .ok()
        .or_else(|| {
            std::env::var("GHOSTTY_LIB_DIR").ok()
        });

    let include_dir = std::env::var("DEP_GHOSTTY_INCLUDE_DIR")
        .ok()
        .or_else(|| {
            std::env::var("GHOSTTY_INCLUDE_DIR").ok()
        });

    let lib_path = lib_dir.as_deref().unwrap_or("/home/josh/Projects/themux/build/libghostty/lib");
    let include_path = include_dir.as_deref().unwrap_or("/home/josh/Projects/themux/build/libghostty/include");

    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=dylib=ghostty-vt");

    let bindings = bindgen::Builder::default()
        .header(format!("{}/ghostty/vt.h", include_path))
        .clang_arg(format!("-I{}", include_path))
        .allowlist_function("ghostty_.*")
        .allowlist_type("Ghostty.*")
        .allowlist_var("GHOSTTY_.*")
        .blocklist_type("^struct Ghostty.*Impl$")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings");
}