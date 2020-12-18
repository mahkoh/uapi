use crate::*;
use std::{convert::TryFrom, mem::MaybeUninit, ptr};

#[man(mount(2))]
pub fn mount<'a, 'b, 'c, 'd>(
    src: impl IntoUstr<'a>,
    target: impl IntoUstr<'b>,
    fstype: impl IntoUstr<'c>,
    flags: c::c_ulong,
    data: Option<&'d [MaybeUninit<u8>]>,
) -> Result<()> {
    let src = src.into_ustr();
    let target = target.into_ustr();
    let fstype = fstype.into_ustr();
    let data = data
        .map(|d| black_box_id(d.as_ptr()) as *const _)
        .unwrap_or(ptr::null());
    let res = unsafe {
        c::mount(
            src.as_ptr(),
            target.as_ptr(),
            fstype.as_ptr_null(),
            flags,
            data,
        )
    };
    map_err!(res).map(drop)
}

#[man(umount2(2))]
pub fn umount2<'a>(target: impl IntoUstr<'a>, flags: c::c_int) -> Result<()> {
    let target = target.into_ustr();
    let res = unsafe { c::umount2(target.as_ptr(), flags) };
    map_err!(res).map(drop)
}

/// [`open_tree(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/namespace.c#L2359)
pub fn open_tree<'a>(
    dfd: c::c_int,
    filename: impl IntoUstr<'a>,
    flags: c::c_uint,
) -> Result<OwnedFd> {
    let filename = filename.into_ustr();
    let res = unsafe { c::open_tree(dfd, filename.as_ptr(), flags) };
    map_err!(res).map(OwnedFd::new)
}

/// [`move_mount(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/namespace.c#L3467-L3479)
pub fn move_mount<'a, 'b>(
    from_dfd: c::c_int,
    from_pathname: impl IntoUstr<'a>,
    to_dfd: c::c_int,
    to_pathname: impl IntoUstr<'b>,
    flags: c::c_uint,
) -> Result<()> {
    let from_pathname = from_pathname.into_ustr();
    let to_pathname = to_pathname.into_ustr();
    let res = unsafe {
        c::move_mount(
            from_dfd,
            from_pathname.as_ptr(),
            to_dfd,
            to_pathname.as_ptr(),
            flags,
        )
    };
    map_err!(res).map(drop)
}

/// [`fsopen(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L107-L115)
pub fn fsopen<'a>(fs_name: impl IntoUstr<'a>, flags: c::c_uint) -> Result<OwnedFd> {
    let fs_name = fs_name.into_ustr();
    let res = unsafe { c::fsopen(fs_name.as_ptr(), flags) };
    map_err!(res).map(OwnedFd::new)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_FLAG`
pub fn fsconfig_set_flag<'a>(fs_fd: c::c_int, key: impl IntoUstr<'a>) -> Result<()> {
    let key = key.into_ustr();
    let res =
        unsafe { c::fsconfig(fs_fd, c::FSCONFIG_SET_FLAG, key.as_ptr(), ptr::null(), 0) };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_STRING`
pub fn fsconfig_set_string<'a, 'b>(
    fs_fd: c::c_int,
    key: impl IntoUstr<'a>,
    value: impl IntoUstr<'b>,
) -> Result<()> {
    let key = key.into_ustr();
    let value = value.into_ustr();
    let res = unsafe {
        c::fsconfig(
            fs_fd,
            c::FSCONFIG_SET_STRING,
            key.as_ptr(),
            value.as_ptr() as *const c::c_void,
            0,
        )
    };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_BINARY`
pub fn fsconfig_set_binary<'a, T: ?Sized>(
    fs_fd: c::c_int,
    key: impl IntoUstr<'a>,
    value: &T,
) -> Result<()> {
    let key = key.into_ustr();
    let value = as_maybe_uninit_bytes(value);
    let len = match c::c_int::try_from(value.len()) {
        Ok(len) => len,
        Err(_) => return Err(Errno(c::EINVAL)),
    };
    let res = unsafe {
        c::fsconfig(
            fs_fd,
            c::FSCONFIG_SET_BINARY,
            key.as_ptr(),
            black_box_id(value.as_ptr()) as *const c::c_void,
            len,
        )
    };
    map_err!(res).map(drop)
}

fn _fsconfig_set_path<'a, 'b>(
    fs_fd: c::c_int,
    cmd: c::c_uint,
    key: impl IntoUstr<'a>,
    dfd: c::c_int,
    path: impl IntoUstr<'b>,
) -> Result<()> {
    let key = key.into_ustr();
    let path = path.into_ustr();
    let res = unsafe {
        c::fsconfig(
            fs_fd,
            cmd,
            key.as_ptr(),
            path.as_ptr() as *const c::c_void,
            dfd,
        )
    };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_PATH`
pub fn fsconfig_set_path<'a, 'b>(
    fs_fd: c::c_int,
    key: impl IntoUstr<'a>,
    dfd: c::c_int,
    path: impl IntoUstr<'b>,
) -> Result<()> {
    _fsconfig_set_path(fs_fd, c::FSCONFIG_SET_PATH, key, dfd, path)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_PATH_EMPTY`
pub fn fsconfig_set_path_empty<'a, 'b>(
    fs_fd: c::c_int,
    key: impl IntoUstr<'a>,
    dfd: c::c_int,
    path: impl IntoUstr<'b>,
) -> Result<()> {
    _fsconfig_set_path(fs_fd, c::FSCONFIG_SET_PATH_EMPTY, key, dfd, path)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_FD`
pub fn fsconfig_set_fd<'a>(
    fs_fd: c::c_int,
    key: impl IntoUstr<'a>,
    fd: c::c_int,
) -> Result<()> {
    let key = key.into_ustr();
    let res =
        unsafe { c::fsconfig(fs_fd, c::FSCONFIG_SET_FD, key.as_ptr(), ptr::null(), fd) };
    map_err!(res).map(drop)
}

fn _fsconfig_cmd(fs_fd: c::c_int, cmd: c::c_uint) -> Result<()> {
    let res = unsafe { c::fsconfig(fs_fd, cmd, ptr::null(), ptr::null(), 0) };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_CMD_CREATE`
pub fn fsconfig_cmd_create(fs_fd: c::c_int) -> Result<()> {
    _fsconfig_cmd(fs_fd, c::FSCONFIG_CMD_CREATE)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_CMD_RECONFIGURE`
pub fn fsconfig_cmd_reconfigure(fs_fd: c::c_int) -> Result<()> {
    _fsconfig_cmd(fs_fd, c::FSCONFIG_CMD_RECONFIGURE)
}

/// [`fsmount(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/namespace.c#L3327-L3333)
pub fn fsmount(
    fs_fd: c::c_int,
    flags: c::c_uint,
    attr_flags: c::c_uint,
) -> Result<OwnedFd> {
    let res = unsafe { c::fsmount(fs_fd, flags, attr_flags) };
    map_err!(res).map(OwnedFd::new)
}

/// [`fspick(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L155-L158)
pub fn fspick<'a>(
    dfd: c::c_int,
    path: impl IntoUstr<'a>,
    flags: c::c_uint,
) -> Result<OwnedFd> {
    let path = path.into_ustr();
    let res = unsafe { c::fspick(dfd, path.as_ptr(), flags) };
    map_err!(res).map(OwnedFd::new)
}
