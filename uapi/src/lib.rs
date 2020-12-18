#![allow(deprecated)]
#![allow(clippy::or_fun_call)]
// https://github.com/rust-lang/rust-clippy/issues/6466
#![allow(clippy::useless_conversion)]
// https://github.com/rust-lang/rust-clippy/issues/6372
#![allow(clippy::transmute_ptr_to_ptr)]

//! Unix API crate
//!
//! NOTE: The crate documentation is in the `docs` module.

extern crate proc; // https://github.com/rust-lang/rust/issues/64450

pub use crate::{
    dir::*, errno::*, fcntl::*, fd::*, file::*, ioctl::*, mount::*, other::*, pod::*,
    poll::*, process::*, result::*, signal::*, socket::*, uninit::*, ustr::*, util::*,
};

use proc::*;

pub mod c;
pub mod docs;

#[macro_use]
mod macros;
mod dir;
mod errno;
mod fcntl;
mod fd;
mod file;
mod ioctl;
mod mount;
mod other;
mod pod;
mod poll;
mod process;
mod result;
mod signal;
mod socket;
mod uninit;
mod ustr;
mod util;
