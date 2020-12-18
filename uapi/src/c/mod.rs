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
            fsconfig, fsmount, fsopen, fspick, ifinfomsg, move_mount, open_tree, renameat2,
            sockaddr_nl, uinput_setup, SYS_fsconfig, SYS_fsmount, SYS_fsopen, SYS_fspick,
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
            UINPUT_IOCTL_BASE, UINPUT_MAX_NAME_SIZE,
        };
    }
}
