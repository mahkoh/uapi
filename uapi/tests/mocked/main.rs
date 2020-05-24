#![allow(unused_imports, dead_code)]

use crate::upstream::{
    dir::*,
    errno::*,
    fcntl::*,
    fd::*,
    file::*,
    ioctl::*,
    mount::*,
    other::*,
    poll::*,
    process::*,
    result::*,
    ustr::{
        bstr::Bstr, bytes::Bytes, into::IntoUstr, read::UapiReadExt, ustr::Ustr,
        ustring::Ustring,
    },
    util::*,
    *,
};
use proc::*;

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
    pub mod process;
    pub mod ioctl;
    pub mod mount;
    pub mod other;
    pub mod poll;
    pub mod result;
    pub mod ustr;
    pub mod util;
}
