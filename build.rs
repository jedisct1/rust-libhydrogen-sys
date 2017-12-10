extern crate bindgen;
extern crate gcc;

use std::env;
use std::path::PathBuf;

fn main() {
    gcc::Build::new().file("libhydrogen/hydrogen.c").compile(
        "hydrogen",
    );

    let bindings = bindgen::Builder::default()
        .header("libhydrogen/hydrogen.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
