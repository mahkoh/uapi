use crate::*;
use std::{mem, mem::MaybeUninit};

#[man(sched_setaffinity(2))]
pub fn sched_setaffinity(pid: c::pid_t, mask: &[usize]) -> Result<()> {
    unsafe {
        let res = c::syscall(
            c::SYS_sched_setaffinity,
            pid as usize,
            mem::size_of_val(mask),
            mask.as_ptr() as usize,
        );
        map_err!(res).map(drop)
    }
}

#[man(sched_getaffinity(2))]
pub fn sched_getaffinity(pid: c::pid_t, mask: &mut [usize]) -> Result<usize> {
    unsafe {
        let res = c::syscall(
            c::SYS_sched_getaffinity,
            pid as usize,
            mem::size_of_val(mask),
            mask.as_mut_ptr() as usize,
        );
        map_err!(res).map(|v| v as usize)
    }
}

#[man(sched_setattr(2))]
pub fn sched_setattr(
    pid: c::pid_t,
    attr: &c::sched_attr,
    flags: c::c_uint,
) -> Result<()> {
    unsafe {
        let mut attr = *attr;
        attr.size = mem::size_of_val(&attr) as u32;
        let res = c::sched_setattr(pid, &mut attr, flags);
        map_err!(res).map(drop)
    }
}

#[man(sched_getattr(2))]
pub fn sched_getattr(pid: c::pid_t, flags: c::c_uint) -> Result<c::sched_attr> {
    unsafe {
        let mut attr = MaybeUninit::uninit();
        let size = mem::size_of_val(&attr) as c::c_uint;
        let res = c::sched_getattr(pid, attr.as_mut_ptr(), size, flags);
        map_err!(res)?;
        Ok(attr.assume_init())
    }
}

#[man(sched_setscheduler(2))]
pub fn sched_setscheduler(
    pid: c::pid_t,
    policy: c::c_int,
    param: &c::sched_param,
) -> Result<c::c_int> {
    unsafe {
        let res = c::sched_setscheduler(pid, policy, param);
        map_err!(res)
    }
}

#[man(sched_getscheduler(2))]
pub fn sched_getscheduler(pid: c::pid_t) -> Result<c::c_int> {
    unsafe {
        let res = c::sched_getscheduler(pid);
        map_err!(res)
    }
}

#[man(sched_setparam(2))]
pub fn sched_setparam(pid: c::pid_t, param: &c::sched_param) -> Result<()> {
    unsafe {
        let res = c::sched_setparam(pid, param);
        map_err!(res).map(drop)
    }
}

#[man(sched_getparam(2))]
pub fn sched_getparam(pid: c::pid_t) -> Result<c::sched_param> {
    unsafe {
        let mut param = MaybeUninit::uninit();
        let res = c::sched_getparam(pid, param.as_mut_ptr());
        map_err!(res)?;
        Ok(param.assume_init())
    }
}

#[man(sched_get_priority_max(2))]
pub fn sched_get_priority_max(policy: c::c_int) -> Result<c::c_int> {
    unsafe {
        let res = c::sched_get_priority_max(policy);
        map_err!(res)
    }
}

#[man(sched_get_priority_min(2))]
pub fn sched_get_priority_min(policy: c::c_int) -> Result<c::c_int> {
    unsafe {
        let res = c::sched_get_priority_min(policy);
        map_err!(res)
    }
}

#[man(sched_rr_get_interval(2))]
pub fn sched_rr_get_interval(pid: c::pid_t) -> Result<c::timespec> {
    unsafe {
        let mut tp = MaybeUninit::uninit();
        let res = c::sched_rr_get_interval(pid, tp.as_mut_ptr());
        map_err!(res)?;
        Ok(tp.assume_init())
    }
}
