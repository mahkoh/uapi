use crate::*;

#[man("fcntl(2) with cmd = `F_ADD_SEALS`")]
pub fn fcntl_add_seals(fd: c::c_int, seals: c::c_int) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_ADD_SEALS, seals) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_GET_SEALS`")]
pub fn fcntl_get_seals(fd: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::fcntl(fd, c::F_GET_SEALS) };
    map_err!(res)
}

#[man("fcntl(2) with cmd = `F_OFD_SETLK`")]
pub fn fcntl_ofd_setlk(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_OFD_SETLK, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_OFD_SETLKW`")]
pub fn fcntl_ofd_setlkw(fd: c::c_int, lock: &c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_OFD_SETLKW, lock) };
    map_err!(res).map(drop)
}

#[man("fcntl(2) with cmd = `F_OFD_GETLK`")]
pub fn fcntl_ofd_getlk(fd: c::c_int, lock: &mut c::flock) -> Result<()> {
    let res = unsafe { c::fcntl(fd, c::F_OFD_GETLK, lock) };
    map_err!(res).map(drop)
}
