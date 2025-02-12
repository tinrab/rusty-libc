#![cfg_attr(not(test), no_std, no_main)]
#![feature(naked_functions)]
#![feature(c_size_t)]
#![feature(c_variadic)]
#![feature(inherent_str_constructors)]
#![feature(decl_macro)]
#![cfg_attr(feature = "start", feature(lang_items))]

// extern crate alloc;

pub mod ffi;
pub mod fmt;
pub mod io;
pub mod memory;
pub mod process;
pub mod signal;
pub mod string;
pub mod sys;

pub mod utility;

#[cfg(not(test))]
mod init;

#[cfg(feature = "start")]
mod start;
