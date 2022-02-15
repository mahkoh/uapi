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
            fsconfig, fsmount, fsopen, fspick, ifinfomsg, move_mount, open_how, open_tree,
            openat2, renameat2, sched_attr, sched_getattr, sched_getparam, sched_getscheduler,
            sched_setattr, sched_setparam, sched_setscheduler, sockaddr_nl, uinput_setup,
            SYS_clone3, SYS_close_range, SYS_epoll_pwait2, SYS_faccessat2, SYS_fsconfig,
            SYS_fsmount, SYS_fsopen, SYS_fspick, SYS_futex_waitv, SYS_io_uring_enter,
            SYS_io_uring_register, SYS_io_uring_setup, SYS_landlock_add_rule,
            SYS_landlock_create_ruleset, SYS_landlock_restrict_self, SYS_memfd_secret,
            SYS_mount_setattr, SYS_move_mount, SYS_open_tree, SYS_openat2, SYS_pidfd_getfd,
            SYS_pidfd_open, SYS_pidfd_send_signal, SYS_process_madvise, SYS_process_mrelease,
            SYS_quotactl_fd, SYS_set_mempolicy_home_node, AT_RECURSIVE, CLOSE_RANGE_CLOEXEC,
            CLOSE_RANGE_UNSHARE, FSCONFIG_CMD_CREATE, FSCONFIG_CMD_RECONFIGURE,
            FSCONFIG_SET_BINARY, FSCONFIG_SET_FD, FSCONFIG_SET_FLAG, FSCONFIG_SET_PATH,
            FSCONFIG_SET_PATH_EMPTY, FSCONFIG_SET_STRING, FSMOUNT_CLOEXEC, FSOPEN_CLOEXEC,
            FSPICK_CLOEXEC, FSPICK_EMPTY_PATH, FSPICK_NO_AUTOMOUNT, FSPICK_SYMLINK_NOFOLLOW,
            MOUNT_ATTR_NOATIME, MOUNT_ATTR_NODEV, MOUNT_ATTR_NODIRATIME, MOUNT_ATTR_NOEXEC,
            MOUNT_ATTR_NOSUID, MOUNT_ATTR_RDONLY, MOUNT_ATTR_RELATIME, MOUNT_ATTR_STRICTATIME,
            MOUNT_ATTR__ATIME, MOVE_MOUNT_F_AUTOMOUNTS, MOVE_MOUNT_F_EMPTY_PATH,
            MOVE_MOUNT_F_SYMLINKS, MOVE_MOUNT_T_AUTOMOUNTS, MOVE_MOUNT_T_EMPTY_PATH,
            MOVE_MOUNT_T_SYMLINKS, MOVE_MOUNT__MASK, OPEN_TREE_CLOEXEC, OPEN_TREE_CLONE,
            RESOLVE_BENEATH, RESOLVE_IN_ROOT, RESOLVE_NO_MAGICLINKS, RESOLVE_NO_SYMLINKS,
            RESOLVE_NO_XDEV, RWF_APPEND, RWF_DSYNC, RWF_HIPRI, RWF_NOWAIT, RWF_SYNC,
            SCHED_FLAG_ALL, SCHED_FLAG_DL_OVERRUN, SCHED_FLAG_KEEP_ALL, SCHED_FLAG_KEEP_PARAMS,
            SCHED_FLAG_KEEP_POLICY, SCHED_FLAG_RECLAIM, SCHED_FLAG_RESET_ON_FORK,
            SCHED_FLAG_UTIL_CLAMP, SCHED_FLAG_UTIL_CLAMP_MAX, SCHED_FLAG_UTIL_CLAMP_MIN,
            UINPUT_IOCTL_BASE, UINPUT_MAX_NAME_SIZE, PIDFD_NONBLOCK,
        };
    }
}
