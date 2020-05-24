use crate::{Bstr, Ustr, Ustring};
use std::{
    ffi::{CStr, CString, OsStr, OsString},
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

macro_rules! foreign_eq {
    ($ty:path) => {
        impl PartialEq<[u8]> for $ty {
            fn eq(&self, other: &[u8]) -> bool {
                Bstr::as_bytes(self) == other
            }
        }

        impl PartialEq<$ty> for [u8] {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == self
            }
        }

        impl PartialEq<str> for $ty {
            fn eq(&self, other: &str) -> bool {
                Bstr::as_bytes(self) == str::as_bytes(other)
            }
        }

        impl PartialEq<$ty> for str {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == str::as_bytes(self)
            }
        }

        impl PartialEq<CStr> for $ty {
            fn eq(&self, other: &CStr) -> bool {
                Bstr::as_bytes(self) == CStr::to_bytes(other)
            }
        }

        impl PartialEq<$ty> for CStr {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == CStr::to_bytes(self)
            }
        }

        impl PartialEq<OsStr> for $ty {
            fn eq(&self, other: &OsStr) -> bool {
                Bstr::as_bytes(self) == OsStr::as_bytes(other)
            }
        }

        impl PartialEq<$ty> for OsStr {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == OsStr::as_bytes(self)
            }
        }

        impl PartialEq<Path> for $ty {
            fn eq(&self, other: &Path) -> bool {
                Bstr::as_bytes(self) == OsStr::as_bytes(Path::as_os_str(other))
            }
        }

        impl PartialEq<$ty> for Path {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == OsStr::as_bytes(Path::as_os_str(self))
            }
        }

        impl PartialEq<Vec<u8>> for $ty {
            fn eq(&self, other: &Vec<u8>) -> bool {
                Bstr::as_bytes(self) == Vec::as_slice(other)
            }
        }

        impl PartialEq<$ty> for Vec<u8> {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == Vec::as_slice(self)
            }
        }

        impl PartialEq<String> for $ty {
            fn eq(&self, other: &String) -> bool {
                Bstr::as_bytes(self) == str::as_bytes(other)
            }
        }

        impl PartialEq<$ty> for String {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == str::as_bytes(self)
            }
        }

        impl PartialEq<CString> for $ty {
            fn eq(&self, other: &CString) -> bool {
                Bstr::as_bytes(self) == CStr::to_bytes(other)
            }
        }

        impl PartialEq<$ty> for CString {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == CStr::to_bytes(self)
            }
        }

        impl PartialEq<OsString> for $ty {
            fn eq(&self, other: &OsString) -> bool {
                Bstr::as_bytes(self) == OsStr::as_bytes(other)
            }
        }

        impl PartialEq<$ty> for OsString {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == OsStr::as_bytes(self)
            }
        }

        impl PartialEq<PathBuf> for $ty {
            fn eq(&self, other: &PathBuf) -> bool {
                Bstr::as_bytes(self) == OsStr::as_bytes(Path::as_os_str(other))
            }
        }

        impl PartialEq<$ty> for PathBuf {
            fn eq(&self, other: &$ty) -> bool {
                Bstr::as_bytes(other) == OsStr::as_bytes(Path::as_os_str(self))
            }
        }
    };
}

foreign_eq!(Bstr);
foreign_eq!(Ustr);
foreign_eq!(Ustring);

macro_rules! mutual_eq {
    ($a:ident, $b:ident) => {
        impl PartialEq<$a> for $b {
            fn eq(&self, other: &$a) -> bool {
                Bstr::as_bytes(self) == Bstr::as_bytes(other)
            }
        }

        impl PartialEq<$b> for $a {
            fn eq(&self, other: &$b) -> bool {
                Bstr::as_bytes(self) == Bstr::as_bytes(other)
            }
        }
    };
}

mutual_eq!(Bstr, Ustr);
mutual_eq!(Bstr, Ustring);
mutual_eq!(Ustr, Ustring);
