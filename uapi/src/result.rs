use crate::get_errno;
use std::{
    fmt,
    fmt::{Display, Formatter},
};

/// `c_int` newtype which wraps `ERRNO` values
///
/// The `Default` implementation returns the current value of `ERRNO`.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Errno(pub crate::c::c_int);

impl std::error::Error for Errno {
}

impl Display for Errno {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Errno> for std::io::Error {
    fn from(e: Errno) -> Self {
        Self::from_raw_os_error(e.0)
    }
}

impl Default for Errno {
    fn default() -> Self {
        Errno(get_errno())
    }
}

pub type Result<T> = std::result::Result<T, Errno>;
