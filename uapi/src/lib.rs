#![allow(deprecated)]

pub use crate::{
    dir::*, errno::*, fcntl::*, fd::*, file::*, ioctl::*, mount::*, other::*, pod::*,
    poll::*, process::*, ptrace::*, result::*, signal::*, socket::*, ustr::*, util::*,
};

use proc::*;

#[cfg(test)]
use testutils::strace;

pub mod c;

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
mod ptrace;
mod result;
mod signal;
mod socket;
mod ustr;
mod util;
