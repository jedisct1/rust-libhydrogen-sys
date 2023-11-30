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
        .allowlist_function("hydro_.*")
        .allowlist_function("randombytes_.*")
        .allowlist_type("hydro_.*")
        .allowlist_type("randombytes_.*")
        .allowlist_var("HYDRO_.*")
        .allowlist_var("hydro_.*")
        .allowlist_var("randombytes_.*")
        .size_t_is_usize(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_eq(true)
        .layout_tests(true)
        .prepend_enum_name(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
