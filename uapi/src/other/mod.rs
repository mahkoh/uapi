use crate::*;
use cfg_if::cfg_if;
use std::{ffi::CStr, mem::MaybeUninit, ops::Deref};

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(umask(2))]
#[notest]
pub fn umask(mask: c::mode_t) -> c::mode_t {
    unsafe { c::umask(mask) }
}

#[man(pipe(2))]
#[notest]
pub fn pipe() -> Result<(OwnedFd, OwnedFd)> {
    let mut buf = [0; 2];
    let res = unsafe { c::pipe(buf.as_mut_ptr()) };
    map_err!(res).map(|_| (OwnedFd::new(buf[0]), OwnedFd::new(buf[1])))
}

/// Wrapper for `libc::utsname`
pub struct UtsName {
    buf: c::utsname,
}

impl UtsName {
    /// Returns `self.sysname` as a `CStr`
    pub fn sysname(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.buf.sysname.as_ptr()) }
    }

    /// Returns `self.nodename` as a `CStr`
    pub fn nodename(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.buf.nodename.as_ptr()) }
    }

    /// Returns `self.release` as a `CStr`
    pub fn release(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.buf.release.as_ptr()) }
    }

    /// Returns `self.version` as a `CStr`
    pub fn version(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.buf.version.as_ptr()) }
    }

    /// Returns `self.machine` as a `CStr`
    pub fn machine(&self) -> &CStr {
        unsafe { CStr::from_ptr(self.buf.machine.as_ptr()) }
    }
}

impl Deref for UtsName {
    type Target = c::utsname;

    fn deref(&self) -> &Self::Target {
        &self.buf
    }
}

#[man(uname(2))]
#[notest]
pub fn uname() -> Result<UtsName> {
    let mut uname = MaybeUninit::uninit();
    let res = unsafe { c::uname(uname.as_mut_ptr()) };
    map_err!(res).map(|_| UtsName {
        buf: unsafe { uname.assume_init() },
    })
}

#[man(daemon(3))]
#[notest]
pub fn daemon(nochdir: bool, noclose: bool) -> Result<()> {
    let res = unsafe { c::daemon(nochdir as _, noclose as _) };
    map_err!(res).map(drop)
}

#[man(sethostname(2))]
#[notest]
pub fn sethostname(buf: &mut [u8]) -> Result<()> {
    let res = unsafe { c::sethostname(buf.as_ptr() as *const _, buf.len()) };
    map_err!(res).map(drop)
}

#[man(gethostname(2))]
#[notest]
///
/// This function returns `libc::ENAMETOOLONG` if the hostname does not fit in the supplied
/// buffer. If the hostname is longer than 255 bytes (excluding the nul byte), then this
/// function always returns `libc::ENAMETOOLONG`.
pub fn gethostname(buf: &mut [u8]) -> Result<&CStr> {
    // Posix implies: If gethostname returns without an error then
    // - if the buffer does not contain a nul byte then the hostname was truncated
    // - if the buffer contains a nul byte in the last place then the hostname was
    //   possibly truncated
    // - otherwise the buffer contains the hostname
    // In either case, the buffer has been fully initialized up to the first nul byte or
    // the end of the buffer depending on what comes first.
    //
    // Therefore strlen on the buffer is defined as long as we manually insert a
    // nul byte at the end of the buffer after gethostname returns
    //
    // SUSv2 guarantees that "Host names are limited to 255 bytes". Presumably this means
    // 255 bytes excluding the terminating nul byte. Therefore we use an
    // internal buffer of size 257 and manually set the last byte to 0 after
    // gethostname returns. If the first nul byte in the buffer is that very byte, then
    // we assume that there was a truncation and return an error.
    unsafe {
        const SIZE: usize = 257;
        let mut inner = MaybeUninit::<[u8; SIZE]>::uninit();
        let res = c::gethostname(inner.as_mut_ptr() as *mut _, SIZE);
        map_err!(res)?;
        *(inner.as_mut_ptr() as *mut u8).add(SIZE - 1) = 0;
        let cstr = CStr::from_ptr(inner.as_ptr() as *mut c::c_char);
        let bytes = cstr.to_bytes_with_nul();
        if bytes.len() < SIZE && bytes.len() <= buf.len() {
            buf[..bytes.len()].copy_from_slice(bytes);
            Ok(CStr::from_bytes_with_nul_unchecked(&buf[..bytes.len()]))
        } else {
            Err(Errno(c::ENAMETOOLONG))
        }
    }
}

#[man(sync(2))]
#[notest]
pub fn sync() {
    unsafe { libc::sync() }
}

#[man(sysconf(2))]
// #[beta]
#[notest]
pub fn sysconf(name: c::c_int) -> Result<c::c_long> {
    set_errno(0);
    let res = unsafe { c::sysconf(name) };
    map_err!(res)
}
