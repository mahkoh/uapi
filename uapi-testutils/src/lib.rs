#![allow(deprecated)]

use std::{
    borrow::Cow,
    fmt,
    fmt::{Debug, Display, Formatter},
};
pub use strace::strace;
use tempfile::TempDir;
use uapi::*;

mod strace;

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
