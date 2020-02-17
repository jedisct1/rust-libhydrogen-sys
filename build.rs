use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .file("libhydrogen/hydrogen.c")
        .flag_if_supported("-fomit-frame-pointer")
        .opt_level(3)
        .compile("hydrogen");

    let bindings = bindgen::Builder::default()
        .header("libhydrogen/hydrogen.h")
        .whitelist_function("hydro_.*")
        .whitelist_function("randombytes_.*")
        .whitelist_type("hydro_.*")
        .whitelist_type("randombytes_.*")
        .whitelist_var("HYDRO_.*")
        .whitelist_var("hydro_.*")
        .whitelist_var("randombytes_.*")
        .size_t_is_usize(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .layout_tests(true)
        .prepend_enum_name(true)
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
