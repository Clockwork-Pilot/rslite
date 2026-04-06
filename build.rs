fn main() {
    // Compile C code: variadic entry points (one file per function)
    cc::Build::new()
        .file("c_code/printf_c.c")
        .file("c_code/snprintf.c")
        .file("c_code/mprintf.c")
        .file("c_code/vsnprintf.c")
        .file("c_code/vmprintf.c")
        .file("c_code/test_control.c")
        .file("c_code/db_config.c")
        .file("c_code/config.c")
        .file("c_code/vtab_config.c")
        .file("c_code/log.c")
        .file("c_code/fts3_errmsg.c")
        .compile("printf_c");

    // Force the linker to pull in C symbols that are only called by external
    // clients (not by Rust code).  Without this, the linker drops unreferenced
    // objects from the static lib.
    #[cfg(not(target_os = "macos"))]
    {
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_snprintf");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_mprintf");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_vsnprintf");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_vmprintf");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_test_control");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_db_config");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_config");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_vtab_config");
        println!("cargo:rustc-link-arg-cdylib=-Wl,--undefined=sqlite3_log");

        // Export C symbols from the cdylib (.so) — Rust's linker only auto-exports
        // #[no_mangle] Rust symbols, so C functions need explicit export directives.
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        let ver_script = format!("{}/c_code/exports.ver", manifest_dir);
        println!("cargo:rustc-cdylib-link-arg=-Wl,--version-script={}", ver_script);
    }
    #[cfg(target_os = "macos")]
    {
        // Force-export C wrapper symbols that the linker would otherwise drop
        for sym in &[
            "sqlite3_snprintf", "sqlite3_mprintf", "sqlite3_vsnprintf",
            "sqlite3_vmprintf", "sqlite3_test_control", "sqlite3_db_config",
            "sqlite3_config", "sqlite3_vtab_config", "sqlite3_log",
        ] {
            println!("cargo:rustc-link-arg-cdylib=-Wl,-exported_symbol,_{}", sym);
        }
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
