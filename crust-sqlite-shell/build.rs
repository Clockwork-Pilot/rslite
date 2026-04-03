use std::env;
use std::path::PathBuf;

fn main() {
    // Find the target directory by navigating up from OUT_DIR
    // OUT_DIR is: {workspace}/target/{profile}/build/crust-sqlite-shell-xxx/out
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    // Go up 3 levels: out -> crust-sqlite-shell-xxx -> build -> {profile}
    let target_profile_dir = out_dir.ancestors().nth(3).unwrap().to_path_buf();

    // Link to the sqlite_noamalgam library built by cargo
    println!("cargo:rustc-link-search=native={}", target_profile_dir.display());
    println!("cargo:rustc-link-lib=sqlite_noamalgam");

    // Enable rpath so the binary finds the library at runtime
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", target_profile_dir.display());

    // Link system libraries that sqlite needs
    println!("cargo:rustc-link-lib=m");      // libm (math)
    println!("cargo:rustc-link-lib=pthread"); // libpthread
    println!("cargo:rustc-link-lib=dl");      // libdl (dynamic loader)
    println!("cargo:rustc-link-lib=z");       // libz (compression)
}
