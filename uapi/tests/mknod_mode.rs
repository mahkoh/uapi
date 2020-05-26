use proc::test_if_root;
use std::{fs::metadata, os::unix::fs::PermissionsExt};
use testutils::Tempdir;
use uapi::*;

#[test_if_root]
fn mknod1() {
    const MODE: c::mode_t = 0o712;

    let f = || {
        let tmp = Tempdir::new();
        let path = format_ustr!("{}/a", tmp);
        mknod(&path, c::S_IFCHR | MODE, 12).unwrap();
        let stat = stat(&path).unwrap();
        assert_eq!(stat.st_rdev, 12);
        metadata(path.as_path()).unwrap().permissions().mode() & 0o777
    };

    umask(0);
    assert_eq!(f(), MODE);
    umask(0o077);
    assert_eq!(f(), 0o700);
}
