#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]
#![allow(static_mut_refs)]

// Force cargo to link sqlite_noamalgam — C FFI calls, no Rust type imports yet.
// Remove once modules use `use sqlite_noamalgam::...` directly.
extern crate sqlite_noamalgam;
#[macro_use]
extern crate c2rust_bitfields;

pub mod src;

fn main() {
    src::src::tclsqlite::main();
}
