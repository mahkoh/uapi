use crate::*;

#[man(setns(2))]
pub fn setns(fd: c::c_int, nstype: c::c_int) -> Result<()> {
    let res = unsafe { c::setns(fd, nstype) };
    map_err!(res).map(drop)
}

#[man(unshare(2))]
pub fn unshare(flags: c::c_int) -> Result<()> {
    let res = unsafe { c::unshare(flags) };
    map_err!(res).map(drop)
}

#[man(execveat(2))]
pub fn execveat<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    argv: &UstrPtr,
    envp: &UstrPtr,
    flags: c::c_int,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let res = unsafe {
        c::syscall(
            c::SYS_execveat,
            dirfd,
            pathname.as_ptr(),
            argv.as_ptr(),
            envp.as_ptr(),
            flags,
        )
    };
    map_err!(res).map(drop)
}

#[man(execvpe(3))]
pub fn execvpe<'a, 'b, 'c>(
    pathname: impl IntoUstr<'a>,
    argv: &UstrPtr,
    envp: &UstrPtr,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let res = unsafe { c::execvpe(pathname.as_ptr(), argv.as_ptr(), envp.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(gettid(2))]
pub fn gettid() -> c::pid_t {
    unsafe { c::syscall(c::SYS_gettid) as _ }
}

#[man(pivot_root(2))]
pub fn pivot_root<'a, 'b>(
    new_root: impl IntoUstr<'a>,
    old_root: impl IntoUstr<'a>,
) -> Result<()> {
    let new_root = new_root.into_ustr();
    let old_root = old_root.into_ustr();
    let res = unsafe {
        c::syscall(
            c::SYS_pivot_root,
            new_root.as_ptr() as usize,
            old_root.as_ptr() as usize,
        )
    };
    map_err!(res).map(drop)
}

#[man(pidfd_open(2))]
pub fn pidfd_open(pid: c::pid_t, flags: c::c_uint) -> Result<OwnedFd> {
    let res = unsafe { c::syscall(c::SYS_pidfd_open, pid as usize, flags as usize) };
    map_err!(res).map(|f| OwnedFd::new(f as _))
}

#[man(pidfd_getfd(2))]
pub fn pidfd_getfd(
    pidfd: c::c_int,
    targetfd: c::c_int,
    flags: c::c_uint,
) -> Result<OwnedFd> {
    let res = unsafe {
        c::syscall(
            c::SYS_pidfd_getfd,
            pidfd as usize,
            targetfd as usize,
            flags as usize,
        )
    };
    map_err!(res).map(|f| OwnedFd::new(f as _))
}
