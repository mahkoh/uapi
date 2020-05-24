use crate::{c::c_char, Ustr, Ustring};
use std::{
    ffi::{CStr, OsStr},
    fmt,
    fmt::{Debug, Display, Formatter},
    ops::{Deref, DerefMut},
    os::unix::ffi::OsStrExt,
    path::Path,
    str::Utf8Error,
};

/// Thin wrapper for a `[u8]`
///
/// See also the crate documentation.
#[repr(transparent)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct Bstr {
    bytes: [u8],
}

impl Bstr {
    /// Returns an empty `&Bstr`
    pub fn empty() -> &'static Self {
        static E: [u8; 0] = [];
        Self::from_bytes(&E[..])
    }

    /// Transmutes the argument into `&Bstr`
    pub fn from_bytes(s: &[u8]) -> &Self {
        unsafe { &*(s as *const _ as *const _) }
    }

    /// Transmutes the argument into `&mut Bstr`
    pub fn from_bytes_mut(s: &mut [u8]) -> &mut Self {
        unsafe { &mut *(s as *mut _ as *mut _) }
    }

    /// Shortcut for `Bstr::from_bytes(s.as_bytes())`
    #[allow(clippy::should_implement_trait)] // https://github.com/rust-lang/rust-clippy/issues/5612
    pub fn from_str(s: &str) -> &Self {
        Self::from_bytes(s.as_bytes())
    }

    /// Shortcut for `Bstr::from_bytes(s.as_bytes())`
    pub fn from_os_str(s: &OsStr) -> &Self {
        Self::from_bytes(s.as_bytes())
    }

    /// Shortcut for `Bstr::from_os_str(s.as_os_str())`
    pub fn from_path(s: &Path) -> &Self {
        Self::from_os_str(s.as_os_str())
    }

    /// Transmutes `self` into `&[u8]`
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Transmutes `self` into `&mut [u8]`
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }

    /// Shortcut for `str::from_utf8(self.as_bytes())`
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        std::str::from_utf8(self.as_bytes())
    }

    /// Shortcut for `OsStr::from_bytes(self.as_bytes())`
    pub fn as_os_str(&self) -> &OsStr {
        OsStr::from_bytes(self.as_bytes())
    }

    /// Shortcut for `Path::new(self.as_os_str())`
    pub fn as_path(&self) -> &Path {
        Path::new(self.as_os_str())
    }

    /// Shortcut for `self.as_bytes().as_ptr()`
    pub fn as_ptr(&self) -> *const c_char {
        self.as_bytes().as_ptr() as *const c_char
    }

    /// Shortcut for `self.as_mut_bytes().as_mut_ptr()`
    pub fn as_mut_ptr(&mut self) -> *mut c_char {
        self.as_bytes_mut().as_mut_ptr() as *mut c_char
    }

    /// Shortcut for `self.as_bytes().len()`
    pub fn len(&self) -> usize {
        self.bytes.len() - 1
    }

    /// Shortcut for `self.len() == 0`
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Shortcut for `self.as_path().display()`
    pub fn display(&self) -> impl Display + '_ {
        self.as_path().display()
    }

    /// Allocates a new `Ustring` with the contents of this object
    pub fn to_ustring(&self) -> Ustring {
        let mut vec = Vec::with_capacity(self.len() + 1);
        vec.extend_from_slice(self.as_bytes());
        vec.push(0);
        unsafe { Ustring::from_vec_with_nul_unchecked(vec) }
    }
}

impl ToOwned for Bstr {
    type Owned = Ustring;

    fn to_owned(&self) -> Self::Owned {
        self.to_ustring()
    }
}

impl Deref for Bstr {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_bytes()
    }
}

impl DerefMut for Bstr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_bytes_mut()
    }
}

impl AsRef<[u8]> for Bstr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for Bstr {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl AsRef<Bstr> for CStr {
    fn as_ref(&self) -> &Bstr {
        Ustr::from_c_str(self).as_bstr()
    }
}

impl AsRef<OsStr> for Bstr {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<Path> for Bstr {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Debug for Bstr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_os_str(), f)
    }
}
