use proc::*;
use std::{fs::metadata, os::unix::fs::PermissionsExt};
use testutils::*;
use uapi::*;

#[test_if(root)]
#[cfg(not(target_os = "macos"))]
fn mknodat1() {
    const MODE: c::mode_t = 0o712;

    let f = || {
        let tmp = Tempdir::new();
        let fd = open(&tmp, c::O_PATH, 0).unwrap();
        let path = format_ustr!("{}/a", tmp);
        mknodat(*fd, "a", c::S_IFCHR | MODE, makedev(1, 2)).unwrap();
        let stat = stat(&path).unwrap();
        assert_eq!(stat.st_rdev, makedev(1, 2));
        metadata(path.as_path()).unwrap().permissions().mode() & 0o777
    };

    umask(0);
    assert_eq!(f(), MODE);
    umask(0o077);
    assert_eq!(f(), 0o700);
}
