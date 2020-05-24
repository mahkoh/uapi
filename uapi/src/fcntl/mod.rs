use crate::*;

#[man("fcntl(2) with cmd = `F_GETFD`")]
#[notest]
pub fn fcntl_getfd(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GETFD) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_SETFD`")]
#[notest]
pub fn fcntl_setfd(fd: c::c_int, flags: c::c_int) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETFD, flags) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_GETFL`")]
#[notest]
pub fn fcntl_getfl(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GETFL) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_SETFL`")]
#[notest]
pub fn fcntl_setfl(fd: c::c_int, flags: c::c_int) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETFL, flags) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_DUPFD`")]
#[notest]
pub fn fcntl_dupfd(fd: c::c_int, lower_bound: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::fcntl(fd, c::F_DUPFD, lower_bound) };
    map_err!(res).map(OwnedFd::new)
}

#[man("fcntl(2) with cmd = `F_DUPFD_CLOEXEC`")]
#[notest]
pub fn fcntl_dupfd_cloexec(fd: c::c_int, lower_bound: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::fcntl(fd, c::F_DUPFD_CLOEXEC, lower_bound) };
    map_err!(res).map(OwnedFd::new)
}

#[man("fcntl(2) with cmd = `F_SETLK`")]
#[notest]
pub fn fcntl_setlk(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_SETLKW`")]
#[notest]
pub fn fcntl_setlkw(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_SETLKW, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_GETLK`")]
#[notest]
pub fn fcntl_getlk(fd: c::c_int, lock: &mut c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_GETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_OFD_SETLK`")]
#[notest]
pub fn fcntl_ofd_setlk(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_OFD_SETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_OFD_SETLKW`")]
#[notest]
pub fn fcntl_ofd_setlkw(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_OFD_SETLKW, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_OFD_GETLK`")]
#[notest]
pub fn fcntl_ofd_getlk(fd: c::c_int, lock: &mut c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_OFD_GETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_ADD_SEALS`")]
#[notest]
pub fn fcntl_add_seals(fd: c::c_int, seals: c::c_int) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_ADD_SEALS, seals) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_GET_SEALS`")]
#[notest]
pub fn fcntl_get_seals(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GET_SEALS) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_SETPIPE_SZ`")]
#[notest]
pub fn fcntl_setpipe_sz(fd: c::c_int, size: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_SETPIPE_SZ, size) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_GETPIPE_SZ`")]
#[notest]
pub fn fcntl_getpipe_sz(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GETPIPE_SZ) };
    map_err!(res)
}
