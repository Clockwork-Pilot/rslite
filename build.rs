fn main() {
    // Enable c_variadic via RUSTFLAGS with -Z unstable-options
    // This allows variadic functions to compile without feature declarations in .rs files
    if let Ok(existing) = std::env::var("RUSTFLAGS") {
        println!("cargo:rustc-env=RUSTFLAGS={} -Z unstable-options -Zcrate-attr=feature(c_variadic)", existing);
    } else {
        println!("cargo:rustc-env=RUSTFLAGS=-Z unstable-options -Zcrate-attr=feature(c_variadic)");
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
