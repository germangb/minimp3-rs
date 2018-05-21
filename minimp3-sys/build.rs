extern crate bindgen;
extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // generate Rust bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-Iminimp3/")
        .generate()
        .expect("Unable to generate bindings");

    // write bindings
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // compile the minimp3 implementation into a library that we link against
    fs::copy("minimp3/minimp3.h", out_path.join("minimp3.c"))
        .expect("Can't copy minimp3 source file");
    cc::Build::new()
        .file(out_path.join("minimp3.c"))
        .define("MINIMP3_IMPLEMENTATION", None)
        .compile("minimp3");
}
