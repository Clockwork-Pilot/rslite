fn main() {
    // TCL library — required for the TCL interpreter embedded in rustfixture
    let tcl = pkg_config::probe_library("tcl").expect("TCL not found via pkg-config");
    for path in &tcl.link_paths {
        println!("cargo:rustc-link-search=native={}", path.display());
    }
    for lib in &tcl.libs {
        println!("cargo:rustc-link-lib={}", lib);
    }

    // System libraries
    println!("cargo:rustc-link-lib=m");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=c");
}
