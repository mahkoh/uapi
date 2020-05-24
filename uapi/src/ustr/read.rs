use crate::Ustring;
use std::{io, io::Read, mem};

/// `Read` extensions
pub trait UapiReadExt {
    /// Like `Read::read_to_end()` but for `Ustring`
    fn read_to_ustring(&mut self, s: &mut Ustring) -> io::Result<usize>;

    /// Shortcut for `read_to_ustring` with a new `Ustring`
    fn read_to_new_ustring(&mut self) -> io::Result<Ustring>;
}

impl<T: Read> UapiReadExt for T {
    fn read_to_ustring(&mut self, orig: &mut Ustring) -> io::Result<usize> {
        let mut s = mem::replace(orig, Ustring::new()).into_vec();
        let res = self.read_to_end(&mut s);
        *orig = Ustring::from_vec(s);
        res
    }

    fn read_to_new_ustring(&mut self) -> io::Result<Ustring> {
        let mut s = Ustring::new();
        self.read_to_ustring(&mut s).map(|_| s)
    }
}
