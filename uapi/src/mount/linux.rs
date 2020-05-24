use crate::{
    c::{
        self, c_int, c_uint, c_void, FSCONFIG_CMD_CREATE, FSCONFIG_CMD_RECONFIGURE,
        FSCONFIG_SET_BINARY, FSCONFIG_SET_FD, FSCONFIG_SET_FLAG, FSCONFIG_SET_PATH,
        FSCONFIG_SET_PATH_EMPTY, FSCONFIG_SET_STRING,
    },
    Errno, IntoUstr, OwnedFd, Result,
};
use std::{convert::TryFrom, ptr};

/// [`open_tree(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/namespace.c#L2359)
pub fn open_tree<'a>(
    dfd: c_int,
    filename: impl IntoUstr<'a>,
    flags: c_uint,
) -> Result<OwnedFd> {
    let filename = filename.into_ustr();
    let res = unsafe { c::open_tree(dfd, filename.as_ptr(), flags) };
    map_err!(res).map(OwnedFd::new)
}

/// [`move_mount(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/namespace.c#L3467-L3479)
pub fn move_mount<'a, 'b>(
    from_dfd: c_int,
    from_pathname: impl IntoUstr<'a>,
    to_dfd: c_int,
    to_pathname: impl IntoUstr<'b>,
    flags: c_uint,
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
pub fn fsopen<'a>(fs_name: impl IntoUstr<'a>, flags: c_uint) -> Result<OwnedFd> {
    let fs_name = fs_name.into_ustr();
    let res = unsafe { c::fsopen(fs_name.as_ptr(), flags) };
    map_err!(res).map(OwnedFd::new)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_FLAG`
pub fn fsconfig_set_flag<'a>(fs_fd: c_int, key: impl IntoUstr<'a>) -> Result<()> {
    let key = key.into_ustr();
    let res =
        unsafe { c::fsconfig(fs_fd, FSCONFIG_SET_FLAG, key.as_ptr(), ptr::null(), 0) };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_STRING`
pub fn fsconfig_set_string<'a, 'b>(
    fs_fd: c_int,
    key: impl IntoUstr<'a>,
    value: impl IntoUstr<'b>,
) -> Result<()> {
    let key = key.into_ustr();
    let value = value.into_ustr();
    let res = unsafe {
        c::fsconfig(
            fs_fd,
            FSCONFIG_SET_STRING,
            key.as_ptr(),
            value.as_ptr() as *const c_void,
            0,
        )
    };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_BINARY`
pub fn fsconfig_set_binary<'a>(
    fs_fd: c_int,
    key: impl IntoUstr<'a>,
    value: &[u8],
) -> Result<()> {
    let key = key.into_ustr();
    let len = match c_int::try_from(value.len()) {
        Ok(len) => len,
        Err(_) => return Err(Errno(c::EINVAL)),
    };
    let res = unsafe {
        c::fsconfig(
            fs_fd,
            FSCONFIG_SET_BINARY,
            key.as_ptr(),
            value.as_ptr() as *const c_void,
            len,
        )
    };
    map_err!(res).map(drop)
}

fn _fsconfig_set_path<'a, 'b>(
    fs_fd: c_int,
    cmd: c_uint,
    key: impl IntoUstr<'a>,
    dfd: c_int,
    path: impl IntoUstr<'b>,
) -> Result<()> {
    let key = key.into_ustr();
    let path = path.into_ustr();
    let res = unsafe {
        c::fsconfig(
            fs_fd,
            cmd,
            key.as_ptr(),
            path.as_ptr() as *const c_void,
            dfd,
        )
    };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_PATH`
pub fn fsconfig_set_path<'a, 'b>(
    fs_fd: c_int,
    key: impl IntoUstr<'a>,
    dfd: c_int,
    path: impl IntoUstr<'b>,
) -> Result<()> {
    _fsconfig_set_path(fs_fd, FSCONFIG_SET_PATH, key, dfd, path)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_PATH_EMPTY`
pub fn fsconfig_set_path_empty<'a, 'b>(
    fs_fd: c_int,
    key: impl IntoUstr<'a>,
    dfd: c_int,
    path: impl IntoUstr<'b>,
) -> Result<()> {
    _fsconfig_set_path(fs_fd, FSCONFIG_SET_PATH_EMPTY, key, dfd, path)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_SET_FD`
pub fn fsconfig_set_fd<'a>(
    fs_fd: c_int,
    key: impl IntoUstr<'a>,
    fd: c_int,
) -> Result<()> {
    let key = key.into_ustr();
    let res =
        unsafe { c::fsconfig(fs_fd, FSCONFIG_SET_FD, key.as_ptr(), ptr::null(), fd) };
    map_err!(res).map(drop)
}

fn _fsconfig_cmd(fs_fd: c_int, cmd: c_uint) -> Result<()> {
    let res = unsafe { c::fsconfig(fs_fd, cmd, ptr::null(), ptr::null(), 0) };
    map_err!(res).map(drop)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_CMD_CREATE`
pub fn fsconfig_cmd_create(fs_fd: c_int) -> Result<()> {
    _fsconfig_cmd(fs_fd, FSCONFIG_CMD_CREATE)
}

/// [`fsconfig(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L271-L320) with cmd = `FSCONFIG_CMD_RECONFIGURE`
pub fn fsconfig_cmd_reconfigure(fs_fd: c_int) -> Result<()> {
    _fsconfig_cmd(fs_fd, FSCONFIG_CMD_RECONFIGURE)
}

/// [`fsmount(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/namespace.c#L3327-L3333)
pub fn fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> Result<OwnedFd> {
    let res = unsafe { c::fsmount(fs_fd, flags, attr_flags) };
    map_err!(res).map(OwnedFd::new)
}

/// [`fspick(2)`](https://github.com/torvalds/linux/blob/v5.6/fs/fsopen.c#L155-L158)
pub fn fspick<'a>(dfd: c_int, path: impl IntoUstr<'a>, flags: c_uint) -> Result<OwnedFd> {
    let path = path.into_ustr();
    let res = unsafe { c::fspick(dfd, path.as_ptr(), flags) };
    map_err!(res).map(OwnedFd::new)
}
