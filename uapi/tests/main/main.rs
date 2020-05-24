#![allow(deprecated)]

extern crate proc; // https://github.com/rust-lang/rust/issues/64450

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
mod ustr;
mod util;
