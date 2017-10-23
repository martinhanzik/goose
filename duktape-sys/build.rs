extern crate gcc;

use std::{env, fs};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("bindings.rs");
    fs::copy("duktape/bindings.rs", out_path)
        .expect("Could not copy bindings to output directory");

    gcc::Build::new()
        .file("duktape/duktape.c")
        .compile("libduktape.a");
}