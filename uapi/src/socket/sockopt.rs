use crate::*;
use std::{TryInto, mem};

#[man(getsockopt(2))]
pub fn getsockopt<T: Pod + ?Sized>(
    sockfd: c::c_int,
    level: c::c_int,
    optname: c::c_int,
    t: &mut T,
) -> Result<usize> {
    let mut len = match mem::size_of_val(t).try_into() {
        Ok(l) => l,
        Err(_) => return einval(),
    };
    let res =
        unsafe { c::getsockopt(sockfd, level, optname, t as *mut _ as *mut _, &mut len) };
    black_box(t);
    map_err!(res).map(|_| len as usize)
}

#[man(setsockopt(2))]
pub fn setsockopt<T: ?Sized>(
    sockfd: c::c_int,
    level: c::c_int,
    optname: c::c_int,
    t: &T,
) -> Result<()> {
    let len = match mem::size_of_val(t).try_into() {
        Ok(l) => l,
        Err(_) => return einval(),
    };
    let t: *const c::c_void = black_box_id(t as *const _ as *const _);
    let res = unsafe { c::setsockopt(sockfd, level, optname, t, len) };
    map_err!(res).map(drop)
}
