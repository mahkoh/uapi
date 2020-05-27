#![allow(non_upper_case_globals, non_camel_case_types)]
#![allow(clippy::unreadable_literal, clippy::missing_safety_doc)]

use crate::{c, c::*};

// https://github.com/rust-lang/libc/pull/1759
pub const SYS_open_tree: c_long = 428;
pub const SYS_move_mount: c_long = 429;
pub const SYS_fsopen: c_long = 430;
pub const SYS_fsconfig: c_long = 431;
pub const SYS_fsmount: c_long = 432;
pub const SYS_fspick: c_long = 433;

// https://github.com/rust-lang/libc/pull/????
pub const OPEN_TREE_CLONE: c_uint = 1;
pub const OPEN_TREE_CLOEXEC: c_uint = O_CLOEXEC as c_uint;

pub const MOVE_MOUNT_F_SYMLINKS: c_uint = 0x00000001;
pub const MOVE_MOUNT_F_AUTOMOUNTS: c_uint = 0x00000002;
pub const MOVE_MOUNT_F_EMPTY_PATH: c_uint = 0x00000004;
pub const MOVE_MOUNT_T_SYMLINKS: c_uint = 0x00000010;
pub const MOVE_MOUNT_T_AUTOMOUNTS: c_uint = 0x00000020;
pub const MOVE_MOUNT_T_EMPTY_PATH: c_uint = 0x00000040;
pub const MOVE_MOUNT__MASK: c_uint = 0x00000077;

pub const FSOPEN_CLOEXEC: c_uint = 0x00000001;

pub const FSPICK_CLOEXEC: c_uint = 0x00000001;
pub const FSPICK_SYMLINK_NOFOLLOW: c_uint = 0x00000002;
pub const FSPICK_NO_AUTOMOUNT: c_uint = 0x00000004;
pub const FSPICK_EMPTY_PATH: c_uint = 0x00000008;

pub const FSCONFIG_SET_FLAG: c_uint = 0;
pub const FSCONFIG_SET_STRING: c_uint = 1;
pub const FSCONFIG_SET_BINARY: c_uint = 2;
pub const FSCONFIG_SET_PATH: c_uint = 3;
pub const FSCONFIG_SET_PATH_EMPTY: c_uint = 4;
pub const FSCONFIG_SET_FD: c_uint = 5;
pub const FSCONFIG_CMD_CREATE: c_uint = 6;
pub const FSCONFIG_CMD_RECONFIGURE: c_uint = 7;

pub const FSMOUNT_CLOEXEC: c_uint = 0x00000001;

pub const MOUNT_ATTR_RDONLY: c_uint = 0x00000001;
pub const MOUNT_ATTR_NOSUID: c_uint = 0x00000002;
pub const MOUNT_ATTR_NODEV: c_uint = 0x00000004;
pub const MOUNT_ATTR_NOEXEC: c_uint = 0x00000008;
pub const MOUNT_ATTR__ATIME: c_uint = 0x00000070;
pub const MOUNT_ATTR_RELATIME: c_uint = 0x00000000;
pub const MOUNT_ATTR_NOATIME: c_uint = 0x00000010;
pub const MOUNT_ATTR_STRICTATIME: c_uint = 0x00000020;
pub const MOUNT_ATTR_NODIRATIME: c_uint = 0x00000080;

pub const AT_RECURSIVE: c_uint = 0x8000;

pub unsafe fn open_tree(dfd: c_int, filename: *const c_char, flags: c_uint) -> c_int {
    syscall(
        SYS_open_tree,
        dfd as usize,
        filename as usize,
        flags as usize,
    ) as c_int
}

pub unsafe fn move_mount(
    from_dfd: c_int,
    from_pathname: *const c_char,
    to_dfd: c_int,
    to_pathname: *const c_char,
    flags: c_uint,
) -> c_int {
    syscall(
        SYS_move_mount,
        from_dfd as usize,
        from_pathname as usize,
        to_dfd as usize,
        to_pathname as usize,
        flags as usize,
    ) as c_int
}

pub unsafe fn fsopen(fs_name: *const c_char, flags: c_uint) -> c_int {
    syscall(SYS_fsopen, fs_name as usize, flags as usize) as c_int
}

pub unsafe fn fsconfig(
    fd: c_int,
    cmd: c_uint,
    key: *const c_char,
    value: *const c_void,
    aux: c_int,
) -> c_int {
    syscall(
        SYS_fsconfig,
        fd as usize,
        cmd as usize,
        key as usize,
        value as usize,
        aux as usize,
    ) as c_int
}

pub unsafe fn fsmount(fs_fd: c_int, flags: c_uint, attr_flags: c_uint) -> c_int {
    syscall(
        SYS_fsmount,
        fs_fd as usize,
        flags as usize,
        attr_flags as usize,
    ) as c_int
}

pub unsafe fn fspick(dfd: c_int, path: *const c_char, flags: c_uint) -> c_int {
    syscall(SYS_fspick, dfd as usize, path as usize, flags as usize) as c_int
}

pub const UINPUT_MAX_NAME_SIZE: usize = 80;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uinput_setup {
    pub id: c::input_id,
    pub name: [c::c_char; UINPUT_MAX_NAME_SIZE],
    pub ff_effects_max: u32,
}

pub const UINPUT_IOCTL_BASE: u8 = b'U';

extern "C" {
    pub fn renameat2(
        olddirfd: c::c_int,
        oldpath: *const c::c_char,
        newdirfd: c::c_int,
        newpath: *const c::c_char,
        flags: c::c_int,
    ) -> c::c_int;
}

pub const RWF_HIPRI: c::c_int = 0x00000001;
pub const RWF_DSYNC: c::c_int = 0x00000002;
pub const RWF_SYNC: c::c_int = 0x00000004;
pub const RWF_NOWAIT: c::c_int = 0x00000008;
pub const RWF_APPEND: c::c_int = 0x00000010;

#[cfg(target_env = "musl")]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip_mreqn {
    pub imr_multiaddr: c::in_addr,
    pub imr_address: c::in_addr,
    pub imr_ifindex: c::c_int,
}
