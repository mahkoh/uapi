use crate::*;
use std::mem::MaybeUninit;

#[man(eventfd(2))]
#[notest]
pub fn eventfd(initval: c::c_uint, flags: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::eventfd(initval, flags) };
    map_err!(res).map(OwnedFd::new)
}

union EventFdUnion {
    num: u64,
    buf: [u8; 8],
}

/// Reads from an eventfd file descriptor
#[notest]
pub fn eventfd_read(fd: c::c_int) -> Result<u64> {
    unsafe {
        let mut buf = EventFdUnion { num: 0 };
        let res = read(fd, &mut buf.buf)?;
        if res < 8 {
            Err(Errno(c::EBADFD))
        } else {
            Ok(buf.num)
        }
    }
}

/// Writes to an eventfd file descriptor
#[notest]
pub fn eventfd_write(fd: c::c_int, num: u64) -> Result<()> {
    unsafe {
        let buf = EventFdUnion { num };
        let res = write(fd, &buf.buf)?;
        if res < 8 {
            Err(Errno(c::EBADFD))
        } else {
            Ok(())
        }
    }
}

#[man(memfd_create(2))]
#[notest]
pub fn memfd_create<'a>(name: impl IntoUstr<'a>, flags: c::c_int) -> Result<OwnedFd> {
    let name = name.into_ustr();
    let res = unsafe { c::syscall(c::SYS_memfd_create, name.as_ptr(), flags) };
    map_err!(res).map(|val| OwnedFd::new(val as _))
}

#[man(sysinfo(2))]
#[notest]
pub fn sysinfo() -> Result<c::sysinfo> {
    let mut sysinfo = MaybeUninit::uninit();
    let res = unsafe { c::sysinfo(sysinfo.as_mut_ptr()) };
    map_err!(res).map(|_| unsafe { sysinfo.assume_init() })
}

#[man(pipe2(2))]
#[notest]
pub fn pipe2(flags: c::c_int) -> Result<(OwnedFd, OwnedFd)> {
    let mut buf = [0; 2];
    let res = unsafe { c::pipe2(buf.as_mut_ptr(), flags) };
    map_err!(res).map(|_| (OwnedFd::new(buf[0]), OwnedFd::new(buf[1])))
}

#[man(syncfs(2))]
#[notest]
pub fn syncfs(fd: c::c_int) -> Result<()> {
    let res = unsafe { libc::syscall(c::SYS_syncfs, fd) };
    map_err!(res).map(drop)
}
