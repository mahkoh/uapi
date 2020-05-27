#![allow(deprecated)]

use std::{
    borrow::Cow,
    fmt,
    fmt::{Debug, Display, Formatter},
    panic::AssertUnwindSafe,
    process::exit,
};
use tempfile::TempDir;
use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(any(target_os = "linux", target_os = "android"))] {
        mod strace;
        pub use strace::strace;
    }
}

#[derive(Debug)]
pub struct Tempdir {
    dir: TempDir,
}

impl Tempdir {
    pub fn new() -> Tempdir {
        Tempdir {
            dir: TempDir::new().unwrap(),
        }
    }

    pub fn bstr(&self) -> &Bstr {
        Bstr::from_path(self.dir.path())
    }
}

impl Default for Tempdir {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoUstr<'a> for &'a Tempdir {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.bstr().into_ustr()
    }
}

impl Display for Tempdir {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.bstr().display().fmt(f)
    }
}

pub fn create_file<'a>(f: impl IntoUstr<'a>) {
    open(f, c::O_CREAT | c::O_RDONLY, 0).unwrap();
}

pub struct Defer<F: FnOnce()>(Option<F>);

impl<F: FnOnce()> Defer<F> {
    pub fn new(f: F) -> Defer<F> {
        Defer(Some(f))
    }
}

impl<F: FnOnce()> Drop for Defer<F> {
    fn drop(&mut self) {
        self.0.take().unwrap()()
    }
}

#[macro_export]
macro_rules! defer {
    ($f:expr) => {
        let _x = crate::Defer::new($f);
    };
}

pub fn in_fork<F: FnOnce()>(f: F) {
    match std::panic::catch_unwind(AssertUnwindSafe(f)) {
        Ok(_) => exit(0),
        Err(_) => exit(1),
    }
}
