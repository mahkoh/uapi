use crate::*;
use std::{borrow::Cow, iter::FromIterator, ptr};

/// Wrapper for a `*const *const libc::c_char` with a terminating null pointer
pub struct UstrPtr<'a> {
    ustrs: Vec<Cow<'a, Ustr>>,
    ptrs: Vec<*const c::c_char>,
}

impl<'a> UstrPtr<'a> {
    /// Creates a new `UstrPtr`
    pub fn new() -> Self {
        Self {
            ustrs: vec![],
            ptrs: vec![ptr::null()],
        }
    }

    /// Appends a `*const libc::c_char`
    pub fn push(&mut self, s: impl IntoUstr<'a>) {
        let s = s.into_ustr();
        self.ustrs.reserve_exact(1);
        self.ptrs.reserve_exact(1);
        self.ptrs.pop();
        self.ptrs.push(s.as_ptr());
        self.ptrs.push(ptr::null());
        self.ustrs.push(s);
    }

    /// Returns the `*const *const c::c_char`
    pub fn as_ptr(&self) -> &*const c::c_char {
        &self.ptrs[0]
    }
}

impl<'a, T: IntoUstr<'a>> Extend<T> for UstrPtr<'a> {
    fn extend<U: IntoIterator<Item = T>>(&mut self, iter: U) {
        for ustr in iter {
            self.push(ustr);
        }
    }
}

impl<'a, T: IntoUstr<'a>> FromIterator<T> for UstrPtr<'a> {
    fn from_iter<U: IntoIterator<Item = T>>(iter: U) -> Self {
        let mut buf = UstrPtr::new();
        buf.extend(iter);
        buf
    }
}

impl<'a> Default for UstrPtr<'a> {
    fn default() -> Self {
        UstrPtr::new()
    }
}
