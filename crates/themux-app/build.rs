#![allow(unused)]

fn main() {
    // Link ghostty-internal library
    println!("cargo:rustc-cdylib-link-args=-l:ghostty-internal");

    println!("cargo:rustc-link-search=/home/josh/Projects/themux/build/libghostty/lib");
    println!("cargo:rustc-link-lib=ghostty-vt");
    println!("cargo:include=/home/josh/Projects/themux/build/libghostty/include");
}
