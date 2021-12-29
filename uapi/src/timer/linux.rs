use crate::*;

#[man(timerfd_create(2))]
pub fn timerfd_create(clockid: c::c_int, flags: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::timerfd_create(clockid, flags) };
    map_err!(res).map(OwnedFd::new)
}

#[man(timerfd_settime(2))]
pub fn timerfd_settime(
    fd: c::c_int,
    flags: c::c_int,
    new_value: &c::itimerspec,
) -> Result<c::itimerspec> {
    let mut old_value = pod::pod_zeroed();
    let res = unsafe { c::timerfd_settime(fd, flags, new_value, &mut old_value) };
    map_err!(res).map(|_| old_value)
}

#[man(timerfd_gettime(2))]
pub fn timerfd_gettime(fd: c::c_int) -> Result<c::itimerspec> {
    let mut curr_value = pod::pod_zeroed();
    let res = unsafe { c::timerfd_gettime(fd, &mut curr_value) };
    map_err!(res).map(|_| curr_value)
}
