use crate::{Bstr, Ustr, Ustring};
use std::{
    borrow::Cow,
    ffi::{CStr, CString, OsStr, OsString},
    fmt::Debug,
    ops::Deref,
    os::unix::ffi::{OsStrExt, OsStringExt},
    path::{Path, PathBuf},
};

/// Trait for objects which can be turned into `Cow<'a, Ustr>`
///
/// # Provided Implementations
///
/// The implementations for `&Ustr` and `Ustring` return `self` unchanged.
///
/// The other provided implementations for borrowed objects first check if the object has a trailing
/// nul byte. If so, this byte is used as the trailing nul byte for the `Ustr`. This means that
/// `IntoUstr` does not guarantee to round-trip. For example
///
/// ```
/// # use uapi::IntoUstr;
/// assert_eq!(b"abc", b"abc\0".into_ustr().as_bytes());
/// ```
pub trait IntoUstr<'a>: Debug {
    /// Converts `self` into `Cow<'a, Ustr>`
    fn into_ustr(self) -> Cow<'a, Ustr>;
}

impl<'a> IntoUstr<'a> for Cow<'a, Ustr> {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self
    }
}

impl<'a> IntoUstr<'a> for &'a Cow<'a, Ustr> {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.deref().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for &'a Ustr {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        Cow::Borrowed(self)
    }
}

impl<'a> IntoUstr<'a> for &'a Ustring {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.deref().into_ustr()
    }
}

impl IntoUstr<'static> for Ustring {
    fn into_ustr(self) -> Cow<'static, Ustr> {
        Cow::Owned(self)
    }
}

impl<'a> IntoUstr<'a> for &'a [u8] {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        if let Some(s) = Ustr::from_bytes(self) {
            return Cow::Borrowed(s);
        }
        Cow::Owned(Ustring::from_vec(self.to_owned()))
    }
}

impl<'a> IntoUstr<'a> for &'a Bstr {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.as_bytes().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for Vec<u8> {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        Cow::Owned(Ustring::from_vec(self))
    }
}

impl<'a> IntoUstr<'a> for &'a str {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.as_bytes().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for String {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.into_bytes().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for &'a CStr {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        Cow::Borrowed(Ustr::from_c_str(self))
    }
}

impl<'a> IntoUstr<'a> for CString {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        Cow::Owned(Ustring::from_c_string(self))
    }
}

impl<'a> IntoUstr<'a> for &'a OsStr {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.as_bytes().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for OsString {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.into_vec().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for &'a Path {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.as_os_str().into_ustr()
    }
}

impl<'a> IntoUstr<'a> for PathBuf {
    fn into_ustr(self) -> Cow<'a, Ustr> {
        self.into_os_string().into_ustr()
    }
}
