use crate::{Bstr, Bytes, Result, Ustr};
use std::{
    borrow::Borrow,
    convert::TryFrom,
    ffi::{CString, OsStr, OsString},
    fmt,
    fmt::{Debug, Formatter},
    mem,
    ops::{Deref, DerefMut},
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    slice,
};

/// Thin wrapper for a `Vec<u8>` that has a trailing nul byte
///
/// NOTE: `Ustring` derefs to `Ustr` derefs to `Bstr` derefs to `[u8]`.
/// Rustdoc might not show all available methods.
///
/// `Ustring` is optimized so that it can be created without allocating.
///
/// See also the crate documentation.
#[derive(Eq, Ord, PartialOrd, PartialEq, Hash)]
pub struct Ustring {
    // invariant: vec is empty or the last element is 0
    bytes: Vec<u8>,
}

impl Ustring {
    /// Creates a new, empty `Ustring`
    ///
    /// NOTE: This function does not allocate
    pub fn new() -> Ustring {
        Ustring { bytes: Vec::new() }
    }

    fn init(&mut self) {
        if self.bytes.is_empty() {
            self.bytes.push(0);
        }
    }

    /// Reserves space for `num` additional bytes in `Ustring`
    ///
    /// See `Vec::reserve`
    pub fn reserve(&mut self, num: usize) {
        self.init();
        self.bytes.reserve(num);
    }

    /// Reserves space for `num` additional bytes in `Ustring`
    ///
    /// See `Vec::reserve_exact`
    pub fn reserve_exact(&mut self, additional: usize) {
        self.init();
        self.bytes.reserve_exact(additional);
    }

    /// Returns the capacity of the underlying Vector excluding the trailing nul byte
    pub fn capacity(&self) -> usize {
        if self.bytes.is_empty() {
            0
        } else {
            self.bytes.capacity() - 1
        }
    }

    /// Passes the unused portion of the underlying vector to a callback
    ///
    /// The size of the slice is `self.capacity() - self.len()`.
    ///
    /// The callback should return the number of bytes written to the the slice.
    /// If the callback returns an error, the `Ustring` is guaranteed to be unchanged.
    ///
    /// # Safety
    ///
    /// - `f` must not read from the slice
    /// - `f` must initialize the slice up to (excluding) the returned index
    pub unsafe fn with_unused<F>(&mut self, f: F) -> Result<usize>
    where
        F: FnOnce(&mut [u8]) -> Result<usize>,
    {
        let mut s = mem::replace(self, Ustring::new());
        s.init();

        let res = {
            let bytes = slice::from_raw_parts_mut(
                s.bytes.as_mut_ptr().add(s.bytes.len() - 1),
                s.bytes.capacity() - s.bytes.len() + 1,
            );
            let len = bytes.len();
            let res = f(&mut bytes[..len - 1]);
            match res {
                Ok(num) => {
                    bytes[num] = 0;
                    s.bytes.set_len(s.bytes.len() + num);
                }
                _ => bytes[0] = 0,
            }
            res
        };

        *self = s;
        res
    }

    /// Turns `s` into an `Ustring` by appending a nul byte.
    pub fn from_vec(mut s: Vec<u8>) -> Self {
        if !s.is_empty() {
            s.push(0);
        }
        Ustring { bytes: s }
    }

    /// Transmutes the argument into `Ustring`
    ///
    /// # Safety
    ///
    /// `s` must be empty or contain a trailing nul byte
    pub unsafe fn from_vec_with_nul_unchecked(s: Vec<u8>) -> Self {
        Ustring { bytes: s }
    }

    /// Checks that `s` is empty or has a trailing nul byte and then turns it into an `Ustring`
    pub fn from_vec_with_nul(s: Vec<u8>) -> std::result::Result<Self, Vec<u8>> {
        if !s.is_empty() && s[s.len() - 1] != 0 {
            return Err(s);
        }
        Ok(Ustring { bytes: s })
    }

    /// Returns the underlying `Vec<u8>` after removing the trailing nul byte
    pub fn into_vec(mut self) -> Vec<u8> {
        if !self.bytes.is_empty() {
            self.bytes.pop();
        }
        self.bytes
    }

    /// Returns the underlying `Vec<u8>` without removing the trailing nul byte
    pub fn into_vec_with_nul(mut self) -> Vec<u8> {
        self.init();
        self.bytes
    }

    /// Shortcut for `Ustring::from_vec(s.into_bytes())`
    pub fn from_string(s: String) -> Self {
        Self::from_vec(s.into_bytes())
    }

    /// Shortcut for `Ustring::from_vec_with_nul(s.into_bytes())`
    pub fn from_string_with_nul(s: String) -> std::result::Result<Self, String> {
        if !s.is_empty() && s.as_bytes()[s.len() - 1] != 0 {
            return Err(s);
        }
        Ok(unsafe { Self::from_vec_with_nul_unchecked(s.into_bytes()) })
    }

    /// Tries to turn `self` into a `String` after removing the trailing nul byte
    pub fn into_string(self) -> std::result::Result<String, Self> {
        String::from_utf8(self.into_vec()).map_err(|e| Self::from_vec(e.into_bytes()))
    }

    /// Tries to turn `self` into a `String` without removing the trailing nul byte
    pub fn into_string_with_nul(self) -> std::result::Result<String, Self> {
        String::from_utf8(self.into_vec_with_nul())
            .map_err(|e| Self::from_vec(e.into_bytes()))
    }

    /// Shortcut for `Ustring::from_vec(s.into_bytes())`
    pub fn from_c_string(s: CString) -> Self {
        Self::from_vec(s.into_bytes())
    }

    /// Tries to turn `self` into a `CString`
    ///
    /// On error, the `usize` is the index of the first interior nul byte.
    pub fn into_c_string(self) -> std::result::Result<CString, (usize, Self)> {
        CString::new(self.into_vec())
            .map_err(|e| (e.nul_position(), Self::from_vec(e.into_vec())))
    }

    /// Shortcut for `Ustring::from_vec(s.into_vec())`
    pub fn from_os_string(s: OsString) -> Self {
        Self::from_vec(s.into_vec())
    }

    /// Shortcut for `Ustring::from_vec_with_nul(s.into_vec())`
    pub fn from_os_string_with_nul(s: OsString) -> std::result::Result<Self, OsString> {
        Self::from_vec_with_nul(s.into_vec()).map_err(OsString::from_vec)
    }

    /// Shortcut for `OsString::from_vec(self.into_vec())`
    pub fn into_os_string(self) -> OsString {
        OsString::from_vec(self.into_vec())
    }

    /// Shortcut for `OsString::from_vec(self.into_vec_with_nul())`
    pub fn into_os_string_with_nul(self) -> OsString {
        OsString::from_vec(self.into_vec_with_nul())
    }

    /// Shortcut for `Ustring::from_os_string(s.into_os_string())`
    pub fn from_path_buf(s: PathBuf) -> Self {
        Self::from_os_string(s.into_os_string())
    }

    /// Shortcut for `Ustring::from_os_string_with_nul(s.into_os_string())`
    pub fn from_path_buf_with_nul(s: PathBuf) -> std::result::Result<Self, PathBuf> {
        Self::from_os_string_with_nul(s.into_os_string()).map_err(PathBuf::from)
    }

    /// Shortcut for `PathBuf::from(self.into_os_string())`
    pub fn into_path_buf(self) -> PathBuf {
        PathBuf::from(self.into_os_string())
    }

    /// Shortcut for `PathBuf::from(self.into_os_string_with_nul())`
    pub fn into_path_buf_with_nul(self) -> PathBuf {
        PathBuf::from(self.into_os_string_with_nul())
    }

    /// Appends the bytes
    ///
    /// For example:
    ///
    /// ```
    /// # use uapi::format_ustr;
    /// let mut s = format_ustr!("hello ");
    /// s.push("world");
    /// assert_eq!("hello world", &s);
    /// ```
    pub fn push<T: Bytes + ?Sized>(&mut self, bytes: &T) {
        let bytes = bytes.bytes();
        if !bytes.is_empty() {
            self.init();
            self.bytes.reserve(bytes.len());
            self.bytes.pop();
            self.bytes.extend_from_slice(bytes);
            self.bytes.push(0);
        }
    }

    /// Returns `self` as a `&Ustr`
    pub fn as_ustr(&self) -> &Ustr {
        if self.bytes.is_empty() {
            Ustr::empty()
        } else {
            unsafe { Ustr::from_bytes_unchecked(&self.bytes) }
        }
    }

    /// Returns `self` as a `&mut Ustr`
    pub fn as_ustr_mut(&mut self) -> &mut Ustr {
        self.init();
        unsafe { Ustr::from_bytes_unchecked_mut(&mut self.bytes) }
    }
}

impl Default for Ustring {
    fn default() -> Self {
        Self::new()
    }
}

impl Borrow<Ustr> for Ustring {
    fn borrow(&self) -> &Ustr {
        self.as_ustr()
    }
}

impl Borrow<Bstr> for Ustring {
    fn borrow(&self) -> &Bstr {
        self.as_ustr().as_bstr()
    }
}

impl From<String> for Ustring {
    fn from(s: String) -> Self {
        Ustring::from_string(s)
    }
}

impl From<Vec<u8>> for Ustring {
    fn from(s: Vec<u8>) -> Self {
        Ustring::from_vec(s)
    }
}

impl From<CString> for Ustring {
    fn from(s: CString) -> Self {
        Ustring::from_c_string(s)
    }
}

impl From<OsString> for Ustring {
    fn from(s: OsString) -> Self {
        Ustring::from_os_string(s)
    }
}

impl From<PathBuf> for Ustring {
    fn from(s: PathBuf) -> Self {
        Ustring::from_path_buf(s)
    }
}

impl TryFrom<Ustring> for String {
    type Error = Ustring;

    fn try_from(value: Ustring) -> std::result::Result<Self, Self::Error> {
        Ustring::into_string(value)
    }
}

impl From<Ustring> for Vec<u8> {
    fn from(s: Ustring) -> Self {
        s.into_vec()
    }
}

impl TryFrom<Ustring> for CString {
    type Error = (usize, Ustring);

    fn try_from(s: Ustring) -> std::result::Result<Self, Self::Error> {
        s.into_c_string()
    }
}

impl From<Ustring> for OsString {
    fn from(s: Ustring) -> Self {
        s.into_os_string()
    }
}

impl From<Ustring> for PathBuf {
    fn from(s: Ustring) -> Self {
        s.into_path_buf()
    }
}

impl AsRef<[u8]> for Ustring {
    fn as_ref(&self) -> &[u8] {
        self.deref().as_ref()
    }
}

impl AsMut<[u8]> for Ustring {
    fn as_mut(&mut self) -> &mut [u8] {
        self.deref_mut().as_mut()
    }
}

impl AsRef<OsStr> for Ustring {
    fn as_ref(&self) -> &OsStr {
        self.deref().as_ref()
    }
}

impl AsRef<Path> for Ustring {
    fn as_ref(&self) -> &Path {
        self.deref().as_ref()
    }
}

impl Deref for Ustring {
    type Target = Ustr;

    fn deref(&self) -> &Self::Target {
        self.as_ustr()
    }
}

impl DerefMut for Ustring {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_ustr_mut()
    }
}

impl Debug for Ustring {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}
