use crate::{Bstr, Ustr};
use std::{
    borrow::Cow,
    ffi::{CStr, OsStr},
    ops::Deref,
    os::unix::ffi::OsStrExt,
    path::Path,
};

/// Trait for objects which can be turned into bytes
///
/// This is mostly an internal API.
pub trait Bytes {
    fn bytes(&self) -> &[u8];
}

impl Bytes for Bstr {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl<'a> Bytes for Cow<'a, Ustr> {
    fn bytes(&self) -> &[u8] {
        self.deref().as_bytes()
    }
}

impl Bytes for [u8] {
    fn bytes(&self) -> &[u8] {
        self
    }
}

impl Bytes for str {
    fn bytes(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Bytes for CStr {
    fn bytes(&self) -> &[u8] {
        self.to_bytes()
    }
}

impl Bytes for OsStr {
    fn bytes(&self) -> &[u8] {
        OsStrExt::as_bytes(self)
    }
}

impl Bytes for Path {
    fn bytes(&self) -> &[u8] {
        OsStrExt::as_bytes(self.as_os_str())
    }
}
