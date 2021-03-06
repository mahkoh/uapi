use crate::*;
use std::mem::MaybeUninit;

#[man(eventfd(2))]
pub fn eventfd(initval: c::c_uint, flags: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::eventfd(initval, flags) };
    map_err!(res).map(OwnedFd::new)
}

/// Reads from an eventfd file descriptor
pub fn eventfd_read(fd: c::c_int) -> Result<u64> {
    let mut num = 0;
    let res = read(fd, &mut num)?.len();
    if res < 8 {
        Err(Errno(c::EBADF))
    } else {
        Ok(num)
    }
}

/// Writes to an eventfd file descriptor
pub fn eventfd_write(fd: c::c_int, num: u64) -> Result<()> {
    let res = write(fd, &num)?;
    if res < 8 {
        Err(Errno(c::EBADF))
    } else {
        Ok(())
    }
}

#[man(memfd_create(2))]
pub fn memfd_create<'a>(name: impl IntoUstr<'a>, flags: c::c_uint) -> Result<OwnedFd> {
    let name = name.into_ustr();
    let res = unsafe {
        c::syscall(c::SYS_memfd_create, name.as_ptr() as usize, flags as usize)
    };
    map_err!(res).map(|val| OwnedFd::new(val as _))
}

#[man(sysinfo(2))]
pub fn sysinfo() -> Result<c::sysinfo> {
    let mut sysinfo = MaybeUninit::uninit();
    let res = unsafe { c::sysinfo(sysinfo.as_mut_ptr()) };
    map_err!(res).map(|_| unsafe { sysinfo.assume_init() })
}

#[man(pipe2(2))]
pub fn pipe2(flags: c::c_int) -> Result<(OwnedFd, OwnedFd)> {
    let mut buf = [0; 2];
    let res = unsafe { c::pipe2(buf.as_mut_ptr(), flags) };
    map_err!(res).map(|_| (OwnedFd::new(buf[0]), OwnedFd::new(buf[1])))
}

#[man(syncfs(2))]
pub fn syncfs(fd: c::c_int) -> Result<()> {
    let res = unsafe { libc::syscall(c::SYS_syncfs, fd as usize) };
    map_err!(res).map(drop)
}
