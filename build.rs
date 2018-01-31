extern crate cc;
extern crate bindgen;

use std::fs;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("minimp3/minimp3.h")
        .generate()
        .expect("Unable to generate minimp3 bindings");

    bindings.write_to_file("src/bindgen.rs").expect("Unable to write bindings to output file");

    fs::copy("minimp3/minimp3.h", "minimp3/minimp3.c").expect("Can't copy minimp3 source file");

    cc::Build::new()
        .file("minimp3/minimp3.c")
        .define("MINIMP3_IMPLEMENTATION", None)
        .define("MINIMP3_NO_WAV", None)
        .compile("minimp3");
}
