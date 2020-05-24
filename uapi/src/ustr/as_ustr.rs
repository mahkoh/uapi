use crate::*;
use std::ffi::{CStr, CString};

/// Used for cheap conversion from into `&Ustr`
pub trait AsUstr {
    /// Perform the conversion
    fn as_ustr(&self) -> &Ustr;
}

impl AsUstr for Ustr {
    fn as_ustr(&self) -> &Ustr {
        self
    }
}

impl AsUstr for Ustring {
    fn as_ustr(&self) -> &Ustr {
        self
    }
}

impl AsUstr for CStr {
    fn as_ustr(&self) -> &Ustr {
        Ustr::from_c_str(self)
    }
}

impl AsUstr for CString {
    fn as_ustr(&self) -> &Ustr {
        Ustr::from_c_str(self)
    }
}
