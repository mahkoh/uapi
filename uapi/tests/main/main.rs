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
mod sched;
mod signal;
mod socket;
mod timer;
mod ustr;
mod util;
