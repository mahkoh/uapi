use crate::*;
use cfg_if::cfg_if;
use std::ptr;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(raise(3))]
pub fn raise(sig: c::c_int) -> Result<()> {
    let res = unsafe { c::raise(sig) };
    map_err!(res).map(drop)
}

/// Returns an empty sig set
pub fn empty_sig_set() -> Result<c::sigset_t> {
    let mut set = pod_zeroed();
    sigemptyset(&mut set)?;
    Ok(set)
}

#[man(sigsetops(3))]
pub fn sigemptyset(set: &mut c::sigset_t) -> Result<()> {
    let res = unsafe { c::sigemptyset(set) };
    map_err!(res).map(drop)
}

#[man(sigsetops(3))]
pub fn sigfillset(set: &mut c::sigset_t) -> Result<()> {
    let res = unsafe { c::sigfillset(set) };
    map_err!(res).map(drop)
}

#[man(sigsetops(3))]
pub fn sigaddset(set: &mut c::sigset_t, signum: c::c_int) -> Result<()> {
    let res = unsafe { c::sigaddset(set, signum) };
    map_err!(res).map(drop)
}

#[man(sigsetops(3))]
pub fn sigdelset(set: &mut c::sigset_t, signum: c::c_int) -> Result<()> {
    let res = unsafe { c::sigdelset(set, signum) };
    map_err!(res).map(drop)
}

#[man(sigsetops(3))]
pub fn sigismember(set: &c::sigset_t, signum: c::c_int) -> Result<bool> {
    let res = unsafe { c::sigismember(set, signum) };
    map_err!(res).map(|_| res == 1)
}

#[man(pthread_sigmask(3))]
pub fn pthread_sigmask(
    how: c::c_int,
    set: Option<&c::sigset_t>,
    oldset: Option<&mut c::sigset_t>,
) -> Result<()> {
    let res = unsafe {
        c::pthread_sigmask(
            how,
            set.map(|v| v as *const _).unwrap_or(ptr::null()),
            oldset.map(|v| v as *mut _).unwrap_or(ptr::null_mut()),
        )
    };
    map_err!(res).map(drop)
}

#[man(sigwait(3))]
pub fn sigwait(set: &c::sigset_t) -> Result<c::c_int> {
    let mut sig = 0;
    let res = unsafe { c::sigwait(set, &mut sig) };
    if res == 0 {
        Ok(sig)
    } else {
        Err(Errno(res))
    }
}

#[man(sigwaitinfo(2))]
#[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
pub fn sigwaitinfo(
    set: &c::sigset_t,
    info: Option<&mut c::siginfo_t>,
) -> Result<c::c_int> {
    let res = unsafe {
        c::sigwaitinfo(set, info.map(|v| v as *mut _).unwrap_or(ptr::null_mut()))
    };
    map_err!(res)
}

#[man(sigtimedwait(2))]
#[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
pub fn sigtimedwait(
    set: &c::sigset_t,
    info: Option<&mut c::siginfo_t>,
    timeout: &c::timespec,
) -> Result<c::c_int> {
    let res = unsafe {
        c::sigtimedwait(
            set,
            info.map(|v| v as *mut _).unwrap_or(ptr::null_mut()),
            timeout,
        )
    };
    map_err!(res)
}
