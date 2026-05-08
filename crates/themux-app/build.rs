#![allow(unused)]

fn main() {
    // Link ghostty-internal library
    println!("cargo:rustc-cdylib-link-args=-l:ghostty-internal");
}