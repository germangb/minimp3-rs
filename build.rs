extern crate gcc;
extern crate bindgen;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("minimp3/minimp3.h")
        .generate()
        .expect("Unable to generate minimp3 bindings");

    bindings
        .write_to_file("src/bindgen.rs")
        .expect("Unable to write bindings to output file");

    gcc::Build::new()
        .file("minimp3/minimp3.h")
        .define("MINIMP3_IMPLEMENTATION", None)
        .define("MINIMP3_NO_WAV", None)
        .compile("minimp3");
}
