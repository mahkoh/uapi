use crate::*;
use cfg_if::cfg_if;
use std::{
    convert::TryFrom,
    io::{IoSlice, IoSliceMut},
    mem::MaybeUninit,
};

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(open(2))]
pub fn open<'a>(
    path: impl IntoUstr<'a>,
    oflag: c::c_int,
    mode: c::mode_t,
) -> Result<OwnedFd> {
    let path = path.into_ustr();
    let val = unsafe { c::open(path.as_ptr(), oflag, mode) };
    map_err!(val).map(OwnedFd::new)
}

#[man(openat(2))]
pub fn openat<'a>(
    dfd: c::c_int,
    path: impl IntoUstr<'a>,
    oflag: c::c_int,
    mode: c::mode_t,
) -> Result<OwnedFd> {
    let path = path.into_ustr();
    let val = unsafe { c::openat(dfd, path.as_ptr(), oflag, mode) };
    map_err!(val).map(OwnedFd::new)
}

#[man(close(2))]
pub fn close(fd: OwnedFd) -> Result<()> {
    let fd = fd.unwrap();
    let val = unsafe { c::close(fd) };
    map_err!(val).map(drop)
}

#[man(read(2))]
pub fn read(fd: c::c_int, buf: &mut [u8]) -> Result<usize> {
    let val = unsafe { c::read(fd, buf.as_mut_ptr() as *mut _, buf.len()) };
    map_err!(val).map(|v| v as usize)
}

#[man(readv(2))]
pub fn readv(fd: c::c_int, bufs: &mut [IoSliceMut<'_>]) -> Result<usize> {
    let len = i32::try_from(bufs.len()).unwrap_or(i32::max_value());
    let val = unsafe { c::readv(fd, bufs.as_mut_ptr() as *mut _, len) };
    map_err!(val).map(|v| v as usize)
}

#[man(pread(2))]
pub fn pread(fd: c::c_int, buf: &mut [u8], offset: c::off_t) -> Result<usize> {
    let val = unsafe { c::pread(fd, buf.as_mut_ptr() as *mut _, buf.len(), offset) };
    map_err!(val).map(|v| v as usize)
}

#[man(preadv(2))]
pub fn preadv(
    fd: c::c_int,
    bufs: &mut [IoSliceMut<'_>],
    offset: c::off_t,
) -> Result<usize> {
    let len = i32::try_from(bufs.len()).unwrap_or(i32::max_value());
    let val = unsafe { c::preadv(fd, bufs.as_mut_ptr() as *mut _, len, offset) };
    map_err!(val).map(|v| v as usize)
}

#[man(dup(2))]
pub fn dup(old: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::dup(old) };
    map_err!(res).map(OwnedFd::new)
}

#[man(dup2(2))]
pub fn dup2(old: c::c_int, new: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::dup2(old, new) };
    map_err!(res)
}

#[man(write(2))]
pub fn write(fd: c::c_int, buf: &[u8]) -> Result<usize> {
    let val = unsafe { c::write(fd, buf.as_ptr() as *const _, buf.len()) };
    map_err!(val).map(|v| v as usize)
}

#[man(pwrite(2))]
pub fn pwrite(fd: c::c_int, buf: &[u8], offset: c::off_t) -> Result<usize> {
    let val = unsafe { c::pwrite(fd, buf.as_ptr() as *const _, buf.len(), offset) };
    map_err!(val).map(|v| v as usize)
}

#[man(writev(2))]
pub fn writev(fd: c::c_int, bufs: &[IoSlice<'_>]) -> Result<usize> {
    let len = i32::try_from(bufs.len()).unwrap_or(i32::max_value());
    let val = unsafe { c::writev(fd, bufs.as_ptr() as *const _, len) };
    map_err!(val).map(|v| v as usize)
}

#[man(pwritev(2))]
pub fn pwritev(fd: c::c_int, bufs: &[IoSlice<'_>], offset: c::off_t) -> Result<usize> {
    let len = i32::try_from(bufs.len()).unwrap_or(i32::max_value());
    let val = unsafe { c::pwritev(fd, bufs.as_ptr() as *const _, len, offset) };
    map_err!(val).map(|v| v as usize)
}

#[man(mknod(2))]
pub fn mknod<'a>(path: impl IntoUstr<'a>, mode: c::mode_t, dev: c::dev_t) -> Result<()> {
    let path = path.into_ustr();
    let val = unsafe { c::mknod(path.as_ptr(), mode, dev) };
    map_err!(val).map(drop)
}

#[man(mknodat(2))]
pub fn mknodat<'a>(
    fd: c::c_int,
    path: impl IntoUstr<'a>,
    mode: c::mode_t,
    dev: c::dev_t,
) -> Result<()> {
    let path = path.into_ustr();
    let val = unsafe { c::mknodat(fd, path.as_ptr(), mode, dev) };
    map_err!(val).map(drop)
}

#[man(readlink(2))]
pub fn readlink<'a>(path: impl IntoUstr<'a>, buf: &mut [u8]) -> Result<usize> {
    let path = path.into_ustr();
    let val =
        unsafe { c::readlink(path.as_ptr(), buf.as_mut_ptr() as *mut _, buf.len()) };
    map_err!(val).map(|e| e as usize)
}

#[man(readlinkat(2))]
pub fn readlinkat<'a>(
    fd: c::c_int,
    path: impl IntoUstr<'a>,
    buf: &mut [u8],
) -> Result<usize> {
    let path = path.into_ustr();
    let val = unsafe {
        c::readlinkat(fd, path.as_ptr(), buf.as_mut_ptr() as *mut _, buf.len())
    };
    map_err!(val).map(|e| e as usize)
}

#[man(fstatat(2))]
pub fn fstatat<'a>(
    fd: c::c_int,
    path: impl IntoUstr<'a>,
    flags: c::c_int,
) -> Result<c::stat> {
    let path = path.into_ustr();
    let mut stat = MaybeUninit::uninit();
    let val = unsafe { c::fstatat(fd, path.as_ptr(), stat.as_mut_ptr(), flags) };
    map_err!(val).map(|_| unsafe { stat.assume_init() })
}

#[man(fstat(2))]
pub fn fstat(fd: c::c_int) -> Result<c::stat> {
    let mut stat = MaybeUninit::uninit();
    let val = unsafe { c::fstat(fd, stat.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { stat.assume_init() })
}

#[man(unlink(2))]
pub fn unlink<'a>(path: impl IntoUstr<'a>) -> Result<()> {
    let path = path.into_ustr();
    let val = unsafe { c::unlink(path.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(unlinkat(2))]
pub fn unlinkat<'a>(
    dfd: c::c_int,
    path: impl IntoUstr<'a>,
    flags: c::c_int,
) -> Result<()> {
    let path = path.into_ustr();
    let val = unsafe { c::unlinkat(dfd, path.as_ptr(), flags) };
    map_err!(val).map(drop)
}

#[man(flock(2))]
pub fn flock(fd: c::c_int, operation: c::c_int) -> Result<()> {
    let val = unsafe { c::flock(fd, operation) };
    map_err!(val).map(drop)
}

#[man(posix_fadvise(2))]
pub fn posix_fadvise(
    fd: c::c_int,
    offset: c::off_t,
    len: c::off_t,
    advice: c::c_int,
) -> Result<()> {
    let val = unsafe { c::posix_fadvise(fd, offset, len, advice) };
    map_err!(val).map(drop)
}

#[man(posix_fallocate(3))]
pub fn posix_fallocate(fd: c::c_int, offset: c::off_t, len: c::off_t) -> Result<()> {
    let val = unsafe { c::posix_fallocate(fd, offset, len) };
    map_err!(val).map(drop)
}

#[man(rename(2))]
pub fn rename<'a, 'b>(
    oldpath: impl IntoUstr<'a>,
    newpath: impl IntoUstr<'b>,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let val = unsafe { c::rename(oldpath.as_ptr(), newpath.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(renameat(2))]
pub fn renameat<'a, 'b>(
    olddirfd: c::c_int,
    oldpath: impl IntoUstr<'a>,
    newdirfd: c::c_int,
    newpath: impl IntoUstr<'b>,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let val =
        unsafe { c::renameat(olddirfd, oldpath.as_ptr(), newdirfd, newpath.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(chmod(2))]
pub fn chmod<'a>(pathname: impl IntoUstr<'a>, mode: c::mode_t) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::chmod(pathname.as_ptr(), mode) };
    map_err!(val).map(drop)
}

#[man(fchmod(2))]
pub fn fchmod(fd: c::c_int, mode: c::mode_t) -> Result<()> {
    let val = unsafe { c::fchmod(fd, mode) };
    map_err!(val).map(drop)
}

#[man(fchmodat(2))]
pub fn fchmodat<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    mode: c::mode_t,
    flags: c::c_int,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::fchmodat(dirfd, pathname.as_ptr(), mode, flags) };
    map_err!(val).map(drop)
}

#[man(stat(2))]
pub fn stat<'a>(pathname: impl IntoUstr<'a>) -> Result<c::stat> {
    let mut stat = MaybeUninit::uninit();
    let pathname = pathname.into_ustr();
    let val = unsafe { c::stat(pathname.as_ptr(), stat.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { stat.assume_init() })
}

#[man(lstat(2))]
pub fn lstat<'a>(pathname: impl IntoUstr<'a>) -> Result<c::stat> {
    let mut stat = MaybeUninit::uninit();
    let pathname = pathname.into_ustr();
    let val = unsafe { c::lstat(pathname.as_ptr(), stat.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { stat.assume_init() })
}

#[man(utimensat(2))]
pub fn utimensat<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    times: &[c::timespec; 2],
    flags: c::c_int,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::utimensat(dirfd, pathname.as_ptr(), times.as_ptr(), flags) };
    map_err!(val).map(drop)
}

#[man(lutimes(3))]
pub fn lutimes<'a>(pathname: impl IntoUstr<'a>, times: &[c::timeval; 2]) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::lutimes(pathname.as_ptr(), times.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(futimens(2))]
pub fn futimens(fd: c::c_int, times: &[c::timespec; 2]) -> Result<()> {
    let val = unsafe { c::futimens(fd, times.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(futimes(3))]
pub fn futimes(fd: c::c_int, times: &[c::timeval; 2]) -> Result<()> {
    let val = unsafe { c::futimes(fd, times.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(mkdir(2))]
pub fn mkdir<'a>(pathname: impl IntoUstr<'a>, mode: c::mode_t) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::mkdir(pathname.as_ptr(), mode) };
    map_err!(val).map(drop)
}

#[man(mkdirat(2))]
pub fn mkdirat<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    mode: c::mode_t,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::mkdirat(dirfd, pathname.as_ptr(), mode) };
    map_err!(val).map(drop)
}

#[man(statvfs(3))]
pub fn statvfs<'a>(path: impl IntoUstr<'a>) -> Result<c::statvfs> {
    let path = path.into_ustr();
    let mut statfs = MaybeUninit::uninit();
    let val = unsafe { c::statvfs(path.as_ptr(), statfs.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { statfs.assume_init() })
}

#[man(fstatvfs(3))]
pub fn fstatvfs(fd: c::c_int) -> Result<c::statvfs> {
    let mut statfs = MaybeUninit::uninit();
    let val = unsafe { c::fstatvfs(fd, statfs.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { statfs.assume_init() })
}

#[man(access(2))]
pub fn access<'a>(pathname: impl IntoUstr<'a>, mode: c::c_int) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::access(pathname.as_ptr(), mode) };
    map_err!(val).map(drop)
}

#[man(faccessat(2))]
pub fn faccessat<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    mode: c::c_int,
    flags: c::c_int,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::faccessat(dirfd, pathname.as_ptr(), mode, flags) };
    map_err!(val).map(drop)
}

#[man(chdir(2))]
pub fn chdir<'a>(path: impl IntoUstr<'a>) -> Result<()> {
    let path = path.into_ustr();
    let val = unsafe { c::chdir(path.as_ptr()) };
    map_err!(val).map(drop)
}

#[man(fchdir(2))]
pub fn fchdir(fd: c::c_int) -> Result<()> {
    let val = unsafe { c::fchdir(fd) };
    map_err!(val).map(drop)
}

#[man(chown(2))]
pub fn chown<'a>(
    pathname: impl IntoUstr<'a>,
    owner: c::uid_t,
    group: c::gid_t,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::chown(pathname.as_ptr(), owner, group) };
    map_err!(val).map(drop)
}

#[man(fchown(2))]
pub fn fchown(fd: c::c_int, owner: c::uid_t, group: c::gid_t) -> Result<()> {
    let val = unsafe { c::fchown(fd, owner, group) };
    map_err!(val).map(drop)
}

#[man(lchown(2))]
pub fn lchown<'a>(
    pathname: impl IntoUstr<'a>,
    owner: c::uid_t,
    group: c::gid_t,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::lchown(pathname.as_ptr(), owner, group) };
    map_err!(val).map(drop)
}

#[man(fchownat(2))]
pub fn fchownat<'a>(
    fd: c::c_int,
    pathname: impl IntoUstr<'a>,
    owner: c::uid_t,
    group: c::gid_t,
    flags: c::c_int,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::fchownat(fd, pathname.as_ptr(), owner, group, flags) };
    map_err!(val).map(drop)
}

#[man(fsync(2))]
pub fn fsync(fd: c::c_int) -> Result<()> {
    let val = unsafe { c::fsync(fd) };
    map_err!(val).map(drop)
}

#[man(fdatasync(2))]
pub fn fdatasync(fd: c::c_int) -> Result<()> {
    let val = unsafe { c::fdatasync(fd) };
    map_err!(val).map(drop)
}

#[man(pathconf(3))]
pub fn pathconf<'a>(path: impl IntoUstr<'a>, name: c::c_int) -> Result<c::c_long> {
    let path = path.into_ustr();
    let val = unsafe { c::pathconf(path.as_ptr(), name) };
    map_err!(val)
}

#[man(fpathconf(3))]
pub fn fpathconf(fd: c::c_int, name: c::c_int) -> Result<c::c_long> {
    let val = unsafe { c::fpathconf(fd, name) };
    map_err!(val)
}

#[man(truncate(2))]
pub fn truncate<'a>(path: impl IntoUstr<'a>, length: c::off_t) -> Result<()> {
    let path = path.into_ustr();
    let val = unsafe { c::truncate(path.as_ptr(), length) };
    map_err!(val).map(drop)
}

#[man(ftruncate(2))]
pub fn ftruncate(fd: c::c_int, length: c::off_t) -> Result<()> {
    let val = unsafe { c::ftruncate(fd, length) };
    map_err!(val).map(drop)
}

#[man(isatty(3))]
pub fn isatty(fd: c::c_int) -> Result<()> {
    let val = unsafe { c::isatty(fd) };
    if val == 1 {
        Ok(())
    } else {
        Err(Errno::default())
    }
}

#[man(link(2))]
pub fn link<'a, 'b>(
    oldpath: impl IntoUstr<'a>,
    newpath: impl IntoUstr<'a>,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let res = unsafe { c::link(oldpath.as_ptr(), newpath.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(linkat(2))]
pub fn linkat<'a, 'b>(
    olddirfd: c::c_int,
    oldpath: impl IntoUstr<'a>,
    newdirfd: c::c_int,
    newpath: impl IntoUstr<'a>,
    flags: c::c_int,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let res = unsafe {
        c::linkat(
            olddirfd,
            oldpath.as_ptr(),
            newdirfd,
            newpath.as_ptr(),
            flags,
        )
    };
    map_err!(res).map(drop)
}

#[man(lseek(2))]
pub fn lseek(fd: c::c_int, offset: c::off_t, whence: c::c_int) -> Result<c::off_t> {
    let res = unsafe { c::lseek(fd, offset, whence) };
    map_err!(res)
}

#[man(mkfifo(3))]
pub fn mkfifo<'a>(pathname: impl IntoUstr<'a>, mode: c::mode_t) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::mkfifo(pathname.as_ptr(), mode) };
    map_err!(val).map(drop)
}

#[man(mkfifoat(3))]
pub fn mkfifoat<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    mode: c::mode_t,
) -> Result<()> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::mkfifoat(dirfd, pathname.as_ptr(), mode) };
    map_err!(val).map(drop)
}

#[man(symlink(2))]
pub fn symlink<'a, 'b>(
    oldpath: impl IntoUstr<'a>,
    newpath: impl IntoUstr<'a>,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let res = unsafe { c::symlink(oldpath.as_ptr(), newpath.as_ptr()) };
    map_err!(res).map(drop)
}

#[man(symlinkat(2))]
pub fn symlinkat<'a, 'b>(
    oldpath: impl IntoUstr<'a>,
    newdirfd: c::c_int,
    newpath: impl IntoUstr<'a>,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let res = unsafe { c::symlinkat(oldpath.as_ptr(), newdirfd, newpath.as_ptr()) };
    map_err!(res).map(drop)
}
