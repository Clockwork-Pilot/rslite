// SQLite Shell - C2Rust transpiled version
// Includes auto-generated Rust code from shell.c

#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]
#![allow(static_mut_refs)]

// Force cargo to link sqlite_noamalgam — shell.rs calls its symbols via C FFI
// so cargo would not detect the dependency automatically without this.
// Remove once shell.rs has real `use sqlite_noamalgam::...` imports.
extern crate sqlite_noamalgam;

mod shell;

// Entry point: delegates to the transpiled main() from shell.rs
fn main() {
    shell::main();
}
