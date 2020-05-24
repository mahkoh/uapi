use crate::*;
use std::mem;

#[man("signalfd(2) with fd = `-1`")]
pub fn signalfd_new(mask: &c::sigset_t, flags: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::signalfd(-1, mask, flags) };
    map_err!(res).map(OwnedFd::new)
}

#[man("signalfd(2) with fd != `-1`")]
pub fn signalfd_mod(fd: c::c_int, mask: &c::sigset_t) -> Result<()> {
    if fd == -1 {
        return Err(Errno(c::EBADF));
    }
    let res = unsafe { c::signalfd(fd, mask, 0) };
    map_err!(res).map(drop)
}

/// Reads from a signalfd file descriptor and returns the elements read
pub fn signalfd_read(
    fd: c::c_int,
    buf: &mut [c::signalfd_siginfo],
) -> Result<&mut [c::signalfd_siginfo]> {
    const SIZE: usize = mem::size_of::<c::signalfd_siginfo>();
    let res = unsafe { c::read(fd, buf as *mut _ as *mut _, SIZE * buf.len()) };
    map_err!(res)?;
    if res as usize % SIZE != 0 {
        return Err(Errno(c::EBADF));
    }
    Ok(&mut buf[..res as usize / SIZE])
}
