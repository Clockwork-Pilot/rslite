#![feature(extern_types, c_variadic)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
#![allow(warnings)]
#![allow(static_mut_refs)]

// Force cargo to link sqlite_noamalgam — speedtest1.rs calls its symbols via C FFI
// so cargo would not detect the dependency automatically without this.
extern crate sqlite_noamalgam;

mod speedtest1;

// Entry point: delegates to the transpiled main() from speedtest1.rs
fn main() {
    speedtest1::main();
}
