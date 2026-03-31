fn main() {
    // Enable c_variadic feature via RUSTFLAGS to avoid declaring it in source
    // The feature is declared only in printf_c_variadic.rs module
    unsafe {
        std::env::set_var("RUSTFLAGS", "-Z unstable-options --cap-lints=warn");
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // add unix dependencies below
        // println!("cargo:rustc-link-lib=readline");
    }

    #[cfg(target_os = "macos")]
    {
        // add macos dependencies below
        // println!("cargo:rustc-link-lib=edit");
    }
}
