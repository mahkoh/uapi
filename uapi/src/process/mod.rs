#![allow(non_snake_case)]

use crate::*;
use std::{convert::TryInto, ffi::CStr, ptr};

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(fork(2))]
///
/// # Safety
///
/// `fork` is unsafe in multi-threaded programs.
///
/// Consider the following example:
///
/// ```rust,ignore
/// #![feature(thread_id_value)]
///
/// use std::sync::atomic::{AtomicI32, AtomicPtr};
/// use std::sync::atomic::Ordering::SeqCst;
/// use std::{thread, ptr};
///
/// pub fn f() {
///     static I: AtomicI32 = AtomicI32::new(0);
///     static D: AtomicPtr<i32> = AtomicPtr::new(ptr::null_mut());
///
///     let data = Box::leak(Box::new(0));
///     let tid = gettid();
///     match I.load(SeqCst) {
///         0 => {
///             I.store(tid, SeqCst);
///             /*
///              * Assumption for safety: No call to `f` can have the same `tid` while we're
///              * in the critical section that spans this comment.
///              */
///             D.store(data, SeqCst);
///         },
///         n if n == tid => {
///             /* D has been set to a valid pointer because of the assumption above */
///             let _v = unsafe { *D.load(SeqCst) };
///         },
///         _ => { },
///     }
/// }
/// ```
///
/// If `fork` is called while another thread is in the critical section, `I` will be
/// initialized in the child but `D` won't. If the child creates a new thread, the new
/// thread might have the same `tid` as the thread in the critical section. Therefore the
/// child thread will dereference a null pointer.
///
/// Consult the upstream documentation.
pub unsafe fn fork() -> Result<c::pid_t> {
    let res = c::fork();
    map_err!(res)
}

#[man(wait(2))]
pub fn wait() -> Result<(c::pid_t, c::c_int)> {
    let mut wstatus = 0;
    let res = unsafe { c::wait(&mut wstatus) };
    map_err!(res).map(|pid| (pid, wstatus))
}

#[man(waitpid(2))]
pub fn waitpid(pid: c::pid_t, options: c::c_int) -> Result<(c::pid_t, c::c_int)> {
    let mut wstatus = 0;
    let res = unsafe { c::waitpid(pid, &mut wstatus, options) };
    map_err!(res).map(|pid| (pid, wstatus))
}

#[man(chroot(2))]
pub fn chroot<'a>(path: impl IntoUstr<'a>) -> Result<()> {
    let path = path.into_ustr();
    let res = unsafe { c::chroot(path.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(execve(2))]
pub fn execve<'a>(
    pathname: impl IntoUstr<'a>,
    argv: &UstrPtr,
    envp: &UstrPtr,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let res = unsafe { c::execve(pathname.as_ptr(), argv.as_ptr(), envp.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(execv(3))]
pub fn execv<'a, 'b>(pathname: impl IntoUstr<'a>, argv: &UstrPtr) -> Result<()> {
    let pathname = pathname.into_ustr();
    let res = unsafe { c::execv(pathname.as_ptr(), argv.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(execvp(3))]
pub fn execvp<'a, 'b>(pathname: impl IntoUstr<'a>, argv: &UstrPtr) -> Result<()> {
    let pathname = pathname.into_ustr();
    let res = unsafe { c::execvp(pathname.as_ptr(), argv.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(fexecve(3))]
#[cfg(not(target_os = "macos"))]
pub fn fexecve(fd: c::c_int, argv: &UstrPtr, envp: &UstrPtr) -> Result<()> {
    let res = unsafe { c::fexecve(fd, argv.as_ptr(), envp.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(getcwd(3))]
pub fn getcwd(buf: &mut [u8]) -> Result<&CStr> {
    let res = unsafe { c::getcwd(buf.as_mut_ptr() as *mut _, buf.len()) };
    if res.is_null() {
        Err(Errno::default())
    } else {
        Ok(unsafe { CStr::from_ptr(res) })
    }
}

#[man(setuid(2))]
pub fn setuid(uid: c::uid_t) -> Result<()> {
    let res = unsafe { c::setuid(uid) };
    map_err!(res).map(drop)
}

#[man(seteuid(2))]
pub fn seteuid(uid: c::uid_t) -> Result<()> {
    let res = unsafe { c::seteuid(uid) };
    map_err!(res).map(drop)
}

#[man(setgid(2))]
pub fn setgid(gid: c::gid_t) -> Result<()> {
    let res = unsafe { c::setgid(gid) };
    map_err!(res).map(drop)
}

#[man(setegid(2))]
pub fn setegid(gid: c::gid_t) -> Result<()> {
    let res = unsafe { c::setegid(gid) };
    map_err!(res).map(drop)
}

#[man(getuid(2))]
pub fn getuid() -> c::uid_t {
    unsafe { c::getuid() }
}

#[man(geteuid(2))]
pub fn geteuid() -> c::uid_t {
    unsafe { c::geteuid() }
}

#[man(getgid(2))]
pub fn getgid() -> c::gid_t {
    unsafe { c::getgid() }
}

#[man(getegid(2))]
pub fn getegid() -> c::gid_t {
    unsafe { c::getegid() }
}

#[man(getgroups(2))]
pub fn getgroups(grouplist: &mut [c::gid_t]) -> Result<&mut [c::gid_t]> {
    let len = grouplist.len().try_into().unwrap_or(c::c_int::max_value());
    let res = unsafe { c::getgroups(len, grouplist.as_mut_ptr()) };
    map_err!(res)?;
    Ok(&mut grouplist[..res as usize])
}

#[man(setgroups(2))]
pub fn setgroups(grouplist: &[c::gid_t]) -> Result<()> {
    let res = unsafe { c::setgroups(grouplist.len(), grouplist.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(getpgrp(2))]
pub fn getpgrp() -> c::pid_t {
    unsafe { c::getpgrp() }
}

#[man(setpgid(2))]
pub fn setpgid(pid: c::pid_t, pgid: c::pid_t) -> Result<()> {
    let res = unsafe { c::setpgid(pid, pgid) };
    map_err!(res).map(drop)
}

#[man(getpid(2))]
pub fn getpid() -> c::pid_t {
    unsafe { c::getpid() }
}

#[man(getppid(2))]
pub fn getppid() -> c::pid_t {
    unsafe { c::getppid() }
}

#[man(setsid(2))]
pub fn setsid() -> Result<c::pid_t> {
    let res = unsafe { c::setsid() };
    map_err!(res)
}

#[man(getsid(2))]
pub fn getsid(pid: c::pid_t) -> c::pid_t {
    unsafe { c::getsid(pid) }
}

#[man(pause(2))]
pub fn pause() {
    unsafe {
        c::pause();
    }
}

#[man(setresuid(2))]
#[cfg(not(target_os = "macos"))]
pub fn setresuid(ruid: c::uid_t, euid: c::uid_t, suid: c::uid_t) -> Result<()> {
    let res = unsafe { c::setresuid(ruid, euid, suid) };
    map_err!(res).map(drop)
}

#[man(setresgid(2))]
#[cfg(not(target_os = "macos"))]
pub fn setresgid(rgid: c::gid_t, egid: c::gid_t, sgid: c::gid_t) -> Result<()> {
    let res = unsafe { c::setresgid(rgid, egid, sgid) };
    map_err!(res).map(drop)
}

#[man(getresuid(2))]
#[cfg(not(target_os = "macos"))]
pub fn getresuid() -> Result<(c::uid_t, c::uid_t, c::uid_t)> {
    let (mut ruid, mut euid, mut suid) = (0, 0, 0);
    let res = unsafe { c::getresuid(&mut ruid, &mut euid, &mut suid) };
    map_err!(res).map(|_| (ruid, euid, suid))
}

#[man(getresgid(2))]
#[cfg(not(target_os = "macos"))]
pub fn getresgid() -> Result<(c::gid_t, c::gid_t, c::gid_t)> {
    let (mut rgid, mut egid, mut sgid) = (0, 0, 0);
    let res = unsafe { c::getresgid(&mut rgid, &mut egid, &mut sgid) };
    map_err!(res).map(|_| (rgid, egid, sgid))
}

#[man(clock_getres(2))]
pub fn clock_getres(clockid: c::clockid_t, tp: &mut c::timespec) -> Result<()> {
    let res = unsafe { c::clock_getres(clockid, tp) };
    map_err!(res).map(drop)
}

#[man(clock_gettime(2))]
pub fn clock_gettime(clockid: c::clockid_t, tp: &mut c::timespec) -> Result<()> {
    let res = unsafe { c::clock_gettime(clockid, tp) };
    map_err!(res).map(drop)
}

#[man(clock_settime(2))]
#[cfg(not(target_os = "macos"))]
pub fn clock_settime(clockid: c::clockid_t, tp: &c::timespec) -> Result<()> {
    let res = unsafe { c::clock_settime(clockid, tp) };
    map_err!(res).map(drop)
}

#[man(clock_nanosleep(2))]
#[cfg(not(target_os = "macos"))]
pub fn clock_nanosleep(
    clockid: c::clockid_t,
    flags: c::c_int,
    tp: &c::timespec,
    remain: Option<&mut c::timespec>,
) -> Result<()> {
    let res = unsafe {
        c::clock_nanosleep(
            clockid,
            flags,
            tp,
            remain.map(|v| v as *mut _).unwrap_or(ptr::null_mut()),
        )
    };
    map_err!(res).map(drop)
}

#[man(kill(2))]
pub fn kill(pid: c::pid_t, sig: c::c_int) -> Result<()> {
    let res = unsafe { c::kill(pid, sig) };
    map_err!(res).map(drop)
}
#[man(wait(2))]
pub fn WEXITSTATUS(s: c::c_int) -> c::c_int {
    unsafe { c::WEXITSTATUS(s) }
}

#[man(wait(2))]
pub fn WTERMSIG(s: c::c_int) -> c::c_int {
    unsafe { c::WTERMSIG(s) }
}

#[man(wait(2))]
pub fn WSTOPSIG(s: c::c_int) -> c::c_int {
    unsafe { c::WSTOPSIG(s) }
}

#[man(wait(2))]
pub fn WIFEXITED(s: c::c_int) -> bool {
    unsafe { c::WIFEXITED(s) }
}

#[man(wait(2))]
pub fn WIFSTOPPED(s: c::c_int) -> bool {
    unsafe { c::WIFSTOPPED(s) }
}

#[man(wait(2))]
pub fn WIFSIGNALED(s: c::c_int) -> bool {
    unsafe { c::WIFSIGNALED(s) }
}

#[man(wait(2))]
pub fn WIFCONTINUED(s: c::c_int) -> bool {
    unsafe { c::WIFCONTINUED(s) }
}

#[man(wait(2))]
pub fn WCOREDUMP(s: c::c_int) -> bool {
    unsafe { c::WCOREDUMP(s) }
}
