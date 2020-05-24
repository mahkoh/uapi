use crate::{c::c_char, Bstr, Ustring};
use std::{
    ffi::{CStr, FromBytesWithNulError, OsStr},
    fmt,
    fmt::{Debug, Formatter},
    ops::{Deref, DerefMut},
    os::unix::ffi::OsStrExt,
    path::Path,
    ptr, slice,
};

/// Thin wrapper for a `[u8]` that has a trailing nul byte
///
/// NOTE: `Ustr` derefs to `Bstr` derefs to `[u8]`. Rustdoc might not show all available methods.
///
/// See also the crate documentation.
#[repr(transparent)]
#[derive(Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct Ustr {
    // invariant: last byte is 0
    bytes: [u8],
}

static NULL: [u8; 1] = [0];

impl Ustr {
    /// Returns an empty `Ustr`
    pub fn empty() -> &'static Self {
        static E: [u8; 1] = [0];
        unsafe { Self::from_bytes_unchecked(&E[..]) }
    }

    /// Returns the unique `&Ustr` for which `self.is_null() == true`
    ///
    /// Apart from `self.is_null()`, the returned value is indistinguishable from  `Ustr::empty()`.
    pub fn null() -> &'static Self {
        unsafe { Self::from_bytes_unchecked(&NULL[..]) }
    }

    /// Returns `true` iff this `Ustr` was constructed via `Ustr::null()`
    pub fn is_null(&self) -> bool {
        self.bytes.as_ptr() == NULL.as_ptr()
    }

    /// Transmutes the argument into `&Ustr`
    ///
    /// # Safety
    ///
    /// `s` must have a trailing nul byte.
    pub unsafe fn from_bytes_unchecked(s: &[u8]) -> &Self {
        &*(s as *const _ as *const _)
    }

    /// Transmutes the argument into `&mut Ustr`
    ///
    /// # Safety
    ///
    /// `s` must have a trailing nul byte.
    pub unsafe fn from_bytes_unchecked_mut(s: &mut [u8]) -> &mut Self {
        &mut *(s as *mut _ as *mut _)
    }

    /// Converts the argument into `&Ustr` after checking that it has a trailing nul byte
    ///
    /// Otherwise `None` is returned.
    pub fn from_bytes(s: &[u8]) -> Option<&Self> {
        if s.is_empty() || s[s.len() - 1] != 0 {
            return None;
        }
        Some(unsafe { Self::from_bytes_unchecked(s) })
    }

    /// Converts the argument into `&mut Ustr` after checking that it has a trailing nul byte
    ///
    /// Otherwise `None` is returned.
    pub fn from_bytes_mut(s: &mut [u8]) -> Option<&mut Self> {
        if s.is_empty() || s[s.len() - 1] != 0 {
            return None;
        }
        Some(unsafe { Self::from_bytes_unchecked_mut(s) })
    }

    /// Transmutes `self` into `&[u8]`
    pub fn as_bytes_with_nul(&self) -> &[u8] {
        &self.bytes
    }

    /// Shortcut for `Ustr::from_bytes(s.as_bytes())`
    #[allow(clippy::should_implement_trait)] // https://github.com/rust-lang/rust-clippy/issues/5612
    pub fn from_str(s: &str) -> Option<&Self> {
        Self::from_bytes(s.as_bytes())
    }

    /// Transmutes the argument into `&Ustr`
    pub fn from_c_str(s: &CStr) -> &Self {
        unsafe { Self::from_bytes_unchecked(s.to_bytes_with_nul()) }
    }

    /// Shortcut for `Ustr::from_c_str(CStr::from_ptr(ptr))`
    ///
    /// # Safety
    ///
    /// Like `CStr::from_ptr`
    pub unsafe fn from_ptr<'a>(ptr: *const c_char) -> &'a Self {
        Self::from_c_str(CStr::from_ptr(ptr))
    }

    /// Shortcut for `Ustr::from_bytes_unchecked_mut(CStr::from_ptr_mut(ptr).as_bytes_with_nul_mut())`
    ///
    /// (`CStr::from_ptr_mut` does not actually exist so check the source if you want to know the
    /// truth.)
    ///
    /// # Safety
    ///
    /// Like `CStr::from_ptr`
    pub unsafe fn from_ptr_mut<'a>(ptr: *mut c_char) -> &'a mut Self {
        let len = Self::from_ptr(ptr).len_with_nul();
        Self::from_bytes_unchecked_mut(slice::from_raw_parts_mut(ptr as *mut _, len))
    }

    /// Shortcut for `CStr::from_bytes_with_nul(self.as_bytes_with_nul())`
    pub fn as_c_str(&self) -> Result<&CStr, FromBytesWithNulError> {
        CStr::from_bytes_with_nul(&self.bytes)
    }

    /// Shortcut for `Ustr::from_bytes(s.as_bytes())`
    pub fn from_os_str(s: &OsStr) -> Option<&Self> {
        Self::from_bytes(s.as_bytes())
    }

    /// Shortcut for `OsStr::from_bytes(self.as_bytes_with_nul())`
    pub fn as_os_str_with_nul(&self) -> &OsStr {
        OsStr::from_bytes(self.as_bytes_with_nul())
    }

    /// Shortcut for `Ustr::from_os_str(s.as_os_str())`
    pub fn from_path(s: &Path) -> Option<&Self> {
        Self::from_os_str(s.as_os_str())
    }

    /// Shortcut for `Path::new(self.as_os_str_with_nul())`
    pub fn as_path_with_nul(&self) -> &Path {
        Path::new(self.as_os_str_with_nul())
    }

    /// Returns the length of the underlying `[u8]` including the trailing nul byte
    pub fn len_with_nul(&self) -> usize {
        self.bytes.len()
    }

    /// Returns the `&Bstr` created by dropping the trailing nul byte
    pub fn as_bstr(&self) -> &Bstr {
        Bstr::from_bytes(&self.bytes[..self.bytes.len() - 1])
    }

    /// Returns the `&mut Bstr` created by dropping the trailing nul byte
    pub fn as_bstr_mut(&mut self) -> &mut Bstr {
        let len = self.bytes.len();
        Bstr::from_bytes_mut(&mut self.bytes[..len - 1])
    }

    /// Returns `ptr::null()` if `self.is_null()`, otherwise `self.as_ptr()`.
    pub fn as_ptr_null(&self) -> *const c_char {
        if self.is_null() {
            ptr::null()
        } else {
            self.as_ptr()
        }
    }
}

impl ToOwned for Ustr {
    type Owned = Ustring;

    fn to_owned(&self) -> Self::Owned {
        self.to_ustring()
    }
}

impl Deref for Ustr {
    type Target = Bstr;

    fn deref(&self) -> &Self::Target {
        self.as_bstr()
    }
}

impl DerefMut for Ustr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_bstr_mut()
    }
}

impl AsRef<[u8]> for Ustr {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl AsMut<[u8]> for Ustr {
    fn as_mut(&mut self) -> &mut [u8] {
        self.as_bytes_mut()
    }
}

impl AsRef<Ustr> for CStr {
    fn as_ref(&self) -> &Ustr {
        Ustr::from_c_str(self)
    }
}

impl AsRef<OsStr> for Ustr {
    fn as_ref(&self) -> &OsStr {
        self.as_os_str()
    }
}

impl AsRef<Path> for Ustr {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl Debug for Ustr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_os_str(), f)
    }
}
