use crate::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man("fcntl(2) with cmd = `F_GETFD`")]
pub fn fcntl_getfd(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GETFD) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_SETFD`")]
pub fn fcntl_setfd(fd: c::c_int, flags: c::c_int) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETFD, flags) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_GETFL`")]
pub fn fcntl_getfl(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GETFL) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_SETFL`")]
pub fn fcntl_setfl(fd: c::c_int, flags: c::c_int) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETFL, flags) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_DUPFD`")]
pub fn fcntl_dupfd(fd: c::c_int, lower_bound: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::fcntl(fd, c::F_DUPFD, lower_bound) };
    map_err!(res).map(OwnedFd::new)
}

#[man("fcntl(2) with cmd = `F_DUPFD_CLOEXEC`")]
pub fn fcntl_dupfd_cloexec(fd: c::c_int, lower_bound: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::fcntl(fd, c::F_DUPFD_CLOEXEC, lower_bound) };
    map_err!(res).map(OwnedFd::new)
}

#[man("fcntl(2) with cmd = `F_SETLK`")]
pub fn fcntl_setlk(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_SETLKW`")]
pub fn fcntl_setlkw(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETLKW, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_GETLK`")]
pub fn fcntl_getlk(fd: c::c_int, lock: &mut c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_GETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_SETPIPE_SZ`")]
#[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
pub fn fcntl_setpipe_sz(fd: c::c_int, size: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_SETPIPE_SZ, size) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_GETPIPE_SZ`")]
#[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
pub fn fcntl_getpipe_sz(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GETPIPE_SZ) };
    map_err!(res)
}
