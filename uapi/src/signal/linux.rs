use crate::*;
use std::mem;

#[man(signalfd(2))]
#[notest]
pub fn signalfd(fd: c::c_int, mask: &c::sigset_t, flags: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::signalfd(fd, mask, flags) };
    map_err!(res).map(OwnedFd::new)
}

/// Reads from an signalfd file descriptor and returns the elements read
#[notest]
pub fn signalfd_read(
    fd: c::c_int,
    buf: &mut [c::signalfd_siginfo],
) -> Result<&mut [c::signalfd_siginfo]> {
    const SIZE: usize = mem::size_of::<c::signalfd_siginfo>();
    let res = unsafe { c::read(fd, buf as *mut _ as *mut _, SIZE * buf.len()) };
    map_err!(res)?;
    if res as usize % SIZE != 0 {
        return Err(Errno(c::EBADFD));
    }
    Ok(&mut buf[..res as usize / SIZE])
}
