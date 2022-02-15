#![allow(non_upper_case_globals, non_camel_case_types)]
#![allow(clippy::unreadable_literal, clippy::missing_safety_doc)]

use crate::{c, c::*};

cfg_if! {
    // https://github.com/torvalds/linux/blob/d567f5db412ed52de0b3b3efca4a451263de6108/arch/alpha/kernel/syscalls/syscall.tbl#L464-L465
    if #[cfg(not(target_arch = "alpha"))] {
        pub const SYS_pidfd_send_signal: c_long = 424;
        pub const SYS_io_uring_setup: c_long = 425;
        pub const SYS_io_uring_enter: c_long = 426;
        pub const SYS_io_uring_register: c_long = 427;
        pub const SYS_open_tree: c_long = 428;
        pub const SYS_move_mount: c_long = 429;
        pub const SYS_fsopen: c_long = 430;
        pub const SYS_fsconfig: c_long = 431;
        pub const SYS_fsmount: c_long = 432;
        pub const SYS_fspick: c_long = 433;
        pub const SYS_pidfd_open: c_long = 434;
        pub const SYS_clone3: c_long = 435;
        pub const SYS_close_range: c_long = 436;
        pub const SYS_openat2: c_long = 437;
        pub const SYS_pidfd_getfd: c_long = 438;
        pub const SYS_faccessat2: c_long = 439;
        pub const SYS_process_madvise: c_long = 440;
        pub const SYS_epoll_pwait2: c_long = 441;
        pub const SYS_mount_setattr: c_long = 442;
        pub const SYS_quotactl_fd: c_long = 443;
        pub const SYS_landlock_create_ruleset: c_long = 444;
        pub const SYS_landlock_add_rule: c_long = 445;
        pub const SYS_landlock_restrict_self: c_long = 446;
        pub const SYS_memfd_secret: c_long = 447;
        pub const SYS_process_mrelease: c_long = 448;
        pub const SYS_futex_waitv: c_long = 449;
        pub const SYS_set_mempolicy_home_node: c_long = 450;
    }
}

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifinfomsg {
    pub ifi_family: c::c_uchar,
    pub ifi_type: c::c_ushort,
    pub ifi_index: c::c_int,
    pub ifi_flags: c::c_uint,
    pub ifi_change: c::c_uint,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_nl {
    pub nl_family: c::sa_family_t,
    pub nl_pad: c::c_ushort,
    pub nl_pid: u32,
    pub nl_groups: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct open_how {
    pub flags: u64,
    pub mode: u64,
    pub resolve: u64,
}

pub const RESOLVE_NO_XDEV: u64 = 0x01;
pub const RESOLVE_NO_MAGICLINKS: u64 = 0x02;
pub const RESOLVE_NO_SYMLINKS: u64 = 0x04;
pub const RESOLVE_BENEATH: u64 = 0x08;
pub const RESOLVE_IN_ROOT: u64 = 0x10;

pub unsafe fn openat2(
    dirfd: c_int,
    pathname: *const c_char,
    how: *mut open_how,
    size: usize,
) -> c_int {
    syscall(
        SYS_openat2,
        dirfd as usize,
        pathname as usize,
        how as usize,
        size,
    ) as c_int
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_attr {
    pub size: u32,
    pub sched_policy: u32,
    pub sched_flags: u64,
    pub sched_nice: i32,
    pub sched_priority: u32,
    pub sched_runtime: u64,
    pub sched_deadline: u64,
    pub sched_period: u64,
    pub sched_util_min: u32,
    pub sched_util_max: u32,
    _private: (),
}

pub unsafe fn sched_getattr(
    pid: pid_t,
    attr: *mut sched_attr,
    size: c::c_uint,
    flags: c::c_uint,
) -> c::c_int {
    syscall(
        SYS_sched_getattr,
        pid as usize,
        attr as usize,
        size as usize,
        flags as usize,
    ) as c::c_int
}

pub unsafe fn sched_setattr(
    pid: pid_t,
    attr: *mut sched_attr,
    flags: c_uint,
) -> c::c_int {
    syscall(
        SYS_sched_setattr,
        pid as usize,
        attr as usize,
        flags as usize,
    ) as c::c_int
}

pub const SCHED_FLAG_RESET_ON_FORK: u64 = 0x01;
pub const SCHED_FLAG_RECLAIM: u64 = 0x02;
pub const SCHED_FLAG_DL_OVERRUN: u64 = 0x04;
pub const SCHED_FLAG_KEEP_POLICY: u64 = 0x08;
pub const SCHED_FLAG_KEEP_PARAMS: u64 = 0x10;
pub const SCHED_FLAG_UTIL_CLAMP_MIN: u64 = 0x20;
pub const SCHED_FLAG_UTIL_CLAMP_MAX: u64 = 0x40;
pub const SCHED_FLAG_KEEP_ALL: u64 = SCHED_FLAG_KEEP_POLICY | SCHED_FLAG_KEEP_PARAMS;
pub const SCHED_FLAG_UTIL_CLAMP: u64 =
    SCHED_FLAG_UTIL_CLAMP_MIN | SCHED_FLAG_UTIL_CLAMP_MAX;
pub const SCHED_FLAG_ALL: u64 = SCHED_FLAG_RESET_ON_FORK
    | SCHED_FLAG_RECLAIM
    | SCHED_FLAG_DL_OVERRUN
    | SCHED_FLAG_KEEP_ALL
    | SCHED_FLAG_UTIL_CLAMP;

// http://lists.busybox.net/pipermail/buildroot/2019-May/250043.html

pub unsafe fn sched_getscheduler(pid: pid_t) -> c::c_int {
    syscall(SYS_sched_getscheduler, pid as usize) as c::c_int
}

pub unsafe fn sched_setscheduler(
    pid: pid_t,
    policy: c_int,
    param: *const sched_param,
) -> c::c_int {
    syscall(
        SYS_sched_setscheduler,
        pid as usize,
        policy as usize,
        param as usize,
    ) as c::c_int
}

pub unsafe fn sched_getparam(pid: pid_t, param: *mut sched_param) -> c_int {
    syscall(SYS_sched_getparam, pid as usize, param as usize) as c::c_int
}

pub unsafe fn sched_setparam(pid: pid_t, param: *const sched_param) -> c_int {
    syscall(SYS_sched_setparam, pid as usize, param as usize) as c::c_int
}

pub const CLOSE_RANGE_UNSHARE: c::c_uint = 1 << 1;
pub const CLOSE_RANGE_CLOEXEC: c::c_uint = 1 << 2;

pub const PIDFD_NONBLOCK: c::c_uint = c::O_NONBLOCK as _;
