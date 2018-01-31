extern crate cc;
extern crate bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("minimp3/minimp3.h")
        //.layout_tests(false)
        .generate()
        .expect("Unable to generate minimp3 bindings");

    bindings
        .write_to_file("src/bindgen.rs")
        .expect("Unable to write bindings to output file");


    cc::Build::new()
        .file("minimp3/minimp3.h")
        .define("MINIMP3_IMPLEMENTATION", None)
        .define("MINIMP3_NO_WAV", None)
        .compile("minimp3");

    //println!("cargo:rustc-link-lib=static=minimp3");
}
