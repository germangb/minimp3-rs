extern crate cc;

fn main() {
    cc::Build::new()
        .include("minimp3/")
        .file("minimp3.c")
        .define("MINIMP3_IMPLEMENTATION", None)
        .compile("minimp3");
}
