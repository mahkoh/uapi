//! Re-export of the libc crate with missing items added
//!
//! Items should be upstreamed if possible.

pub use libc::*;

use cfg_if::cfg_if;

#[cfg(target_os = "dragonfly")]
extern "C" {
    pub fn __errno_location() -> *mut c_int;
}

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::{
            fsconfig, fsmount, fsopen, fspick, ifinfomsg, move_mount, open_tree, renameat2, openat2,
            sockaddr_nl, uinput_setup, open_how, SYS_fsconfig, SYS_fsmount, SYS_fsopen, SYS_fspick,
            SYS_move_mount, SYS_open_tree, AT_RECURSIVE, FSCONFIG_CMD_CREATE,
            FSCONFIG_CMD_RECONFIGURE, FSCONFIG_SET_BINARY, FSCONFIG_SET_FD, FSCONFIG_SET_FLAG,
            FSCONFIG_SET_PATH, FSCONFIG_SET_PATH_EMPTY, FSCONFIG_SET_STRING, FSMOUNT_CLOEXEC,
            FSOPEN_CLOEXEC, FSPICK_CLOEXEC, FSPICK_EMPTY_PATH, FSPICK_NO_AUTOMOUNT,
            FSPICK_SYMLINK_NOFOLLOW, MOUNT_ATTR_NOATIME, MOUNT_ATTR_NODEV, MOUNT_ATTR_NODIRATIME,
            MOUNT_ATTR_NOEXEC, MOUNT_ATTR_NOSUID, MOUNT_ATTR_RDONLY, MOUNT_ATTR_RELATIME,
            MOUNT_ATTR_STRICTATIME, MOUNT_ATTR__ATIME, MOVE_MOUNT_F_AUTOMOUNTS,
            MOVE_MOUNT_F_EMPTY_PATH, MOVE_MOUNT_F_SYMLINKS, MOVE_MOUNT_T_AUTOMOUNTS,
            MOVE_MOUNT_T_EMPTY_PATH, MOVE_MOUNT_T_SYMLINKS, MOVE_MOUNT__MASK, OPEN_TREE_CLOEXEC,
            OPEN_TREE_CLONE, RWF_APPEND, RWF_DSYNC, RWF_HIPRI, RWF_NOWAIT, RWF_SYNC,
            UINPUT_IOCTL_BASE, UINPUT_MAX_NAME_SIZE, RESOLVE_NO_XDEV, RESOLVE_NO_MAGICLINKS,
            RESOLVE_NO_SYMLINKS, RESOLVE_BENEATH, RESOLVE_IN_ROOT,
            SYS_pidfd_open, SYS_clone3, SYS_close_range, SYS_openat2, SYS_pidfd_getfd,
            SYS_faccessat2, SYS_process_madvise, sched_attr, sched_setattr, sched_getattr,
            SCHED_FLAG_RESET_ON_FORK, SCHED_FLAG_RECLAIM, SCHED_FLAG_DL_OVERRUN,
            SCHED_FLAG_KEEP_POLICY, SCHED_FLAG_KEEP_PARAMS, SCHED_FLAG_UTIL_CLAMP_MIN,
            SCHED_FLAG_UTIL_CLAMP_MAX, SCHED_FLAG_KEEP_ALL, SCHED_FLAG_UTIL_CLAMP,
            SCHED_FLAG_ALL, sched_getscheduler, sched_setscheduler, sched_getparam,
            sched_setparam, CLOSE_RANGE_CLOEXEC, CLOSE_RANGE_UNSHARE,
        };
    }
}
