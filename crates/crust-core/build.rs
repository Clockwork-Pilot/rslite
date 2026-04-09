fn main() {
    // CARGO_MANIFEST_DIR = <root>/crates/source; c_code lives two levels up at <root>/c_code.
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = std::path::Path::new(&manifest_dir)
        .join("../..")
        .canonicalize()
        .unwrap();
    let c_code = root.join("c_code");

    // Compile C code: variadic entry points (one file per function)
    cc::Build::new()
        .file(c_code.join("printf_c.c"))
        .file(c_code.join("snprintf.c"))
        .file(c_code.join("mprintf.c"))
        .file(c_code.join("vsnprintf.c"))
        .file(c_code.join("vmprintf.c"))
        .file(c_code.join("test_control.c"))
        .file(c_code.join("db_config.c"))
        .file(c_code.join("config.c"))
        .file(c_code.join("vtab_config.c"))
        .file(c_code.join("log.c"))
        .file(c_code.join("fts3_errmsg.c"))
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
        let ver_script = c_code.join("exports.ver");
        println!(
            "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
            ver_script.display()
        );
    }
    #[cfg(target_os = "macos")]
    {
        // Force-export C wrapper symbols that the linker would otherwise drop
        for sym in &[
            "sqlite3_snprintf",
            "sqlite3_mprintf",
            "sqlite3_vsnprintf",
            "sqlite3_vmprintf",
            "sqlite3_test_control",
            "sqlite3_db_config",
            "sqlite3_config",
            "sqlite3_vtab_config",
            "sqlite3_log",
        ] {
            println!("cargo:rustc-link-arg-cdylib=-Wl,-exported_symbol,_{}", sym);
        }
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // add unix dependencies below
        // println!("cargo:rustc-link-lib=readline");
    }

    #[cfg(feature = "test")]
    {
        let sqlite_src = std::env::var("SQLITE_SRC").unwrap_or_else(|_| "/sqlite".to_string());
        let tsrc = std::path::Path::new(&sqlite_src).join("tsrc");
        let fts3_src = std::path::Path::new(&sqlite_src).join("ext/fts3/fts3_term.c");
        cc::Build::new()
            .file(&fts3_src)
            .include(&tsrc)
            .define("SQLITE_TEST", None)
            .define("SQLITE_ENABLE_FTS3", None)
            .define("SQLITE_CORE", None)
            .compile("fts3_term");
        println!("cargo:rustc-link-lib=tcl8.6");
    }

    #[cfg(target_os = "macos")]
    {
        // add macos dependencies below
        // println!("cargo:rustc-link-lib=edit");
    }
}
