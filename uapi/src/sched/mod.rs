use crate::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

fn check_errno(res: c::c_int) -> Result<c::c_int> {
    if res == -1 {
        let errno = get_errno();
        if errno == 0 {
            Ok(-1)
        } else {
            Err(Errno(errno))
        }
    } else {
        Ok(res)
    }
}

#[man(nice(2))]
pub fn nice(inc: c::c_int) -> Result<c::c_int> {
    set_errno(0);
    let res = unsafe { c::nice(inc) };
    check_errno(res)
}

#[man(getpriority(2))]
pub fn getpriority(which: c::c_int, who: c::id_t) -> Result<c::c_int> {
    set_errno(0);
    let res = unsafe { c::getpriority(which as _, who as _) };
    check_errno(res)
}

#[man(setpriority(2))]
pub fn setpriority(which: c::c_int, who: c::id_t, prio: c::c_int) -> Result<()> {
    unsafe {
        let res = c::setpriority(which as _, who as _, prio);
        map_err!(res).map(drop)
    }
}

#[man(sched_yield(2))]
pub fn sched_yield() -> Result<()> {
    unsafe {
        let res = c::sched_yield();
        map_err!(res).map(drop)
    }
}
