#![allow(non_snake_case)]

use crate::*;
use std::ptr;

#[man(wait(2))]
#[notest]
pub const fn WEXITSTATUS(s: c::c_int) -> c::c_int {
    (s & 0xff00) >> 8
}

#[man(wait(2))]
#[notest]
pub const fn WTERMSIG(s: c::c_int) -> c::c_int {
    s & 0x7f
}

#[man(wait(2))]
#[notest]
pub const fn WSTOPSIG(s: c::c_int) -> c::c_int {
    (s & 0xff00) >> 8
}

#[man(wait(2))]
#[notest]
pub const fn WIFEXITED(s: c::c_int) -> bool {
    #[allow(clippy::verbose_bit_mask)]
    {
        s & 0x7f == 0
    }
}

#[man(wait(2))]
#[notest]
pub const fn WIFSTOPPED(s: c::c_int) -> bool {
    s & 0xff == 0x7f
}

#[man(wait(2))]
#[notest]
pub fn WIFSIGNALED(s: c::c_int) -> bool {
    (s & 0x7f != 0) && (s & 0x7f != 0x7f)
}

#[man(wait(2))]
#[notest]
pub const fn WIFCONTINUED(s: c::c_int) -> bool {
    s == 0xffff
}

#[man(wait(2))]
#[notest]
pub const fn WCOREDUMP(s: c::c_int) -> bool {
    s & 0x80 != 0
}

#[man(setns(2))]
#[notest]
pub fn setns(fd: c::c_int, nstype: c::c_int) -> Result<()> {
    let res = unsafe { c::setns(fd, nstype) };
    map_err!(res).map(drop)
}

#[man(unshare(2))]
#[notest]
pub fn unshare(flags: c::c_int) -> Result<()> {
    let res = unsafe { c::unshare(flags) };
    map_err!(res).map(drop)
}

#[man(execveat(2))]
#[notest]
pub fn execveat<'a, 'b, 'c>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    argv: impl Iterator<Item = &'a Ustr>,
    envp: impl Iterator<Item = &'b Ustr>,
    flags: c::c_int,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let mut argv: Vec<_> = argv.map(|v| v.as_ptr()).collect();
    let mut envp: Vec<_> = envp.map(|v| v.as_ptr()).collect();
    argv.push(ptr::null());
    envp.push(ptr::null());
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
#[notest]
pub fn execvpe<'a, 'b, 'c>(
    pathname: impl IntoUstr<'a>,
    argv: impl Iterator<Item = &'a Ustr>,
    envp: impl Iterator<Item = &'b Ustr>,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let mut argv: Vec<_> = argv.map(|v| v.as_ptr()).collect();
    let mut envp: Vec<_> = envp.map(|v| v.as_ptr()).collect();
    argv.push(ptr::null());
    envp.push(ptr::null());
    let res = unsafe { c::execvpe(pathname.as_ptr(), argv.as_ptr(), envp.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(gettid(2))]
#[notest]
pub fn gettid() -> c::pid_t {
    unsafe { c::syscall(c::SYS_gettid) as _ }
}

#[man(pivot_root(2))]
#[notest]
pub fn pivot_root<'a, 'b>(
    new_root: impl IntoUstr<'a>,
    old_root: impl IntoUstr<'a>,
) -> Result<()> {
    let new_root = new_root.into_ustr();
    let old_root = old_root.into_ustr();
    let res =
        unsafe { c::syscall(c::SYS_pivot_root, new_root.as_ptr(), old_root.as_ptr()) };
    map_err!(res).map(drop)
}
