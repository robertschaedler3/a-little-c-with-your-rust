use std::env;
use std::path::PathBuf;

use cmake::Config;

fn main() {
    let dst = Config::new("libhello").build();

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search=native={}", dst.display());

    // Tell cargo to tell rustc to link to our shared library.
    println!("cargo:rustc-link-lib=static=hello");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("libhello/hello.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
