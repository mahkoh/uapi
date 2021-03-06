use crate::*;
use std::{convert::TryInto, ptr};

#[man(epoll_create1(2))]
pub fn epoll_create1(flags: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::epoll_create1(flags) };
    map_err!(res).map(OwnedFd::new)
}

#[man(epoll_ctl(2))]
pub fn epoll_ctl(
    epfd: c::c_int,
    op: c::c_int,
    fd: c::c_int,
    event: Option<&c::epoll_event>,
) -> Result<()> {
    let res = unsafe {
        c::epoll_ctl(
            epfd,
            op,
            fd,
            event.map(|v| v as *const _).unwrap_or(ptr::null()) as *mut _,
        )
    };
    map_err!(res).map(drop)
}

#[man(epoll_wait(2))]
pub fn epoll_wait(
    epfd: c::c_int,
    events: &mut [c::epoll_event],
    timeout: c::c_int,
) -> Result<usize> {
    let len = events.len().try_into().unwrap_or(c::c_int::max_value());
    let res = unsafe { c::epoll_wait(epfd, events.as_mut_ptr(), len, timeout) };
    map_err!(res).map(|v| v as usize)
}
