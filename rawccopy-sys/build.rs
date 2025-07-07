fn main() {
    // Link the shlwapi library
    println!("cargo:rustc-link-lib=shlwapi");
    // Invalidate the build whenever any of the included header files changed.
    println!("cargo:rerun-if-changed=rawccopy/rawccopy");

    // Compile the C code
    cc::Build::new()
        .files([
            "rawccopy/rawccopy/attribs.c",
            "rawccopy/rawccopy/byte-buffer.c",
            "rawccopy/rawccopy/context.c",
            "rawccopy/rawccopy/data-writer.c",
            "rawccopy/rawccopy/disk-info.c",
            "rawccopy/rawccopy/fileio.c",
            "rawccopy/rawccopy/helpers.c",
            "rawccopy/rawccopy/index.c",
            "rawccopy/rawccopy/mft.c",
            "rawccopy/rawccopy/network.c",
            "rawccopy/rawccopy/path.c",
            "rawccopy/rawccopy/processor.c",
            "rawccopy/rawccopy/regex.c",
            "rawccopy/rawccopy/safe-string.c",
            "rawccopy/rawccopy/settings.c",
            "rawccopy/rawccopy/rawccopy_api.c",
        ])
        .include("rawccopy/rawccopy")
        .compile("rawccopy");

    // Generate the Rust bindings
    let bindings = bindgen::Builder::default()
        // The header files
        .header("rawccopy/rawccopy/context.h")
        .header("rawccopy/rawccopy/rawccopy_api.h")
        .header("rawccopy/rawccopy/processor.h")
        // Tell bindgen to only generate bindings for used functions
        .allowlist_function("SetupContext")
        .allowlist_function("CleanUp")
        .allowlist_function("PerformOperation")
        .allowlist_function("rawccopy_open")
        .allowlist_function("rawccopy_read")
        .allowlist_function("rawccopy_close")
        .allowlist_function("rawccopy_stream")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
