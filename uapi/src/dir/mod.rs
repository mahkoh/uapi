use crate::*;
use std::{
    ffi::CStr,
    mem,
    ops::{Deref, DerefMut},
};

#[man(opendir(3))]
pub fn opendir<'a>(name: impl IntoUstr<'a>) -> Result<Dir> {
    let name = name.into_ustr();
    let dir = unsafe { c::opendir(name.as_ptr()) };
    if dir.is_null() {
        Err(Errno::default())
    } else {
        Ok(Dir { dir })
    }
}

#[man(fdopendir(3))]
pub fn fdopendir(fd: OwnedFd) -> Result<Dir> {
    let fd = fd.unwrap();
    let dir = unsafe { c::fdopendir(fd) };
    if dir.is_null() {
        Err(Errno::default())
    } else {
        Ok(Dir { dir })
    }
}

#[man(readdir(3))]
#[allow(clippy::should_implement_trait)] // https://github.com/rust-lang/rust-clippy/issues/5004
pub fn readdir<'a>(dir: &'a mut c::DIR) -> Option<Result<Dirent<'a>>> {
    set_errno(0);
    let ent = unsafe { c::readdir(dir) };
    if ent.is_null() {
        if get_errno() == 0 {
            None
        } else {
            Some(Err(Errno::default()))
        }
    } else {
        unsafe { Some(Ok(Dirent { raw: &*ent })) }
    }
}

#[man(rewinddir(3))]
pub fn rewinddir(dir: &mut c::DIR) {
    unsafe { c::rewinddir(dir) }
}

#[man(seekdir(3))]
pub fn seekdir(dir: &mut c::DIR, loc: c::c_long) {
    unsafe { c::seekdir(dir, loc) }
}

#[man(telldir(3))]
pub fn telldir(dir: &mut c::DIR) -> c::c_long {
    unsafe { c::telldir(dir) }
}

#[man(dirfd(3))]
pub fn dirfd(dir: &mut c::DIR) -> c::c_int {
    unsafe { c::dirfd(dir) }
}

/// Wrapper for `*mut libc::DIR`
///
/// Upon `Drop`, the `*mut libc::DIR` will be closed.
pub struct Dir {
    dir: *mut c::DIR,
}

#[allow(clippy::needless_lifetimes)]
impl Dir {
    /// Unwraps the underlying `*mut libc::DIR`
    pub fn unwrap(self) -> *mut c::DIR {
        let res = self.dir;
        mem::forget(self);
        res
    }

    /// Wraps the provided `*mut libc::DIR`
    ///
    /// # Safety
    ///
    /// The pointer must be valid and `Self` acquires sole ownership
    pub unsafe fn from_ptr(dir: *mut c::DIR) -> Self {
        Self { dir }
    }
}

impl Drop for Dir {
    fn drop(&mut self) {
        unsafe {
            c::closedir(self.dir);
        }
    }
}

impl Deref for Dir {
    type Target = c::DIR;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.dir }
    }
}

impl DerefMut for Dir {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.dir }
    }
}

/// Wrapper for `*const libc::dirent`
pub struct Dirent<'a> {
    raw: &'a c::dirent,
}

impl<'a> Dirent<'a> {
    /// Returns `dirent.d_name` as a `CStr`
    pub fn name(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.raw.d_name.as_ptr()) }
    }
}

impl<'a> Deref for Dirent<'a> {
    type Target = c::dirent;

    fn deref(&self) -> &Self::Target {
        self.raw
    }
}
