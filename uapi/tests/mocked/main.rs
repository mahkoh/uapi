#![allow(unused_imports, dead_code)]

use crate::upstream::{
    dir::*, errno::*, fcntl::*, fd::*, file::*, ioctl::*, mount::*, other::*, pod::*,
    poll::*, process::*, ptrace::*, result::*, signal::*, socket::*, ustr::*, util::*,
};
use proc::*;
use testutils::*;

mod c;

#[rustfmt::skip] // https://github.com/rust-lang/rustfmt/pull/4194
#[path = "../../src"]
mod upstream {
    #[macro_use]
    pub mod macros;
    pub mod dir;
    pub mod errno;
    pub mod fcntl;
    pub mod fd;
    pub mod file;
    pub mod ioctl;
    pub mod mount;
    pub mod other;
    pub mod pod;
    pub mod poll;
    pub mod process;
    pub mod ptrace;
    pub mod result;
    pub mod signal;
    pub mod socket;
    pub mod ustr;
    pub mod util;
}
