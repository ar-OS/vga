#![feature(const_fn)]
#![feature(ptr_internals)]
#![no_std]

extern crate spin;
extern crate volatile;
#[macro_use]
extern crate lazy_static;

pub mod buffer;
pub mod color;
