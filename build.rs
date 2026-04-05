fn main() {
    // Enable c_variadic via RUSTFLAGS with -Z unstable-options
    // This allows variadic functions to compile without feature declarations in .rs files
    if let Ok(existing) = std::env::var("RUSTFLAGS") {
        println!("cargo:rustc-env=RUSTFLAGS={} -Z unstable-options -Zcrate-attr=feature(c_variadic)", existing);
    } else {
        println!("cargo:rustc-env=RUSTFLAGS=-Z unstable-options -Zcrate-attr=feature(c_variadic)");
    }

    // Compile C code: variadic entry points (one file per function)
    cc::Build::new()
        .file("c_code/printf_c.c")
        .file("c_code/snprintf.c")
        .compile("printf_c");

    // Force the linker to pull in C symbols that are only called by external
    // clients (not by Rust code).  Without this, the linker drops unreferenced
    // objects from the static lib.
    println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_snprintf");

    // Export C symbols from the cdylib (.so) — Rust's linker only auto-exports
    // #[no_mangle] Rust symbols, so C functions need explicit export directives.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let ver_script = format!("{}/c_code/exports.ver", manifest_dir);
    println!("cargo:rustc-cdylib-link-arg=-Wl,--version-script={}", ver_script);

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
