use crate::*;
use std::{
    borrow::{Cow},
    iter::FromIterator,
    ptr,
};

pub struct UstrPtr<'a> {
    ustrs: Vec<Cow<'a, Ustr>>,
    ptrs: Vec<*const c::c_char>,
}

impl<'a> UstrPtr<'a> {
    pub fn new() -> Self {
        Self {
            ustrs: vec![],
            ptrs: vec![ptr::null()],
        }
    }

    pub fn push(&mut self, s: impl IntoUstr<'a>) {
        let s = s.into_ustr();
        self.ustrs.reserve_exact(1);
        self.ptrs.reserve_exact(1);
        self.ptrs.pop();
        self.ptrs.push(s.as_ptr());
        self.ptrs.push(ptr::null());
        self.ustrs.push(s);
    }

    pub fn as_ptr(&self) -> *const *const c::c_char {
        self.ptrs.as_ptr()
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
