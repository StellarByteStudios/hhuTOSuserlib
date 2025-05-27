#![no_std]
#![allow(dead_code)] // avoid warnings
#![allow(unused_variables)] // avoid warnings

extern crate alloc;
pub mod kernel;
pub mod utility;

#[macro_use] // import macros, too
pub mod graphix;

fn main() {}

// Kompiler Errors
#[cfg(all(not(feature = "global-alloc"), feature = "runtime"))]
compile_error!("Feature `runtime` requires `global-alloc` to be enabled.");