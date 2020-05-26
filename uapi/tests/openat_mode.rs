use std::{fs::metadata, os::unix::fs::PermissionsExt};
use testutils::*;
use uapi::*;

#[test]
fn openat1() {
    const MODE: c::mode_t = 0o712;

    let f = || {
        let tmp = Tempdir::new();
        let dir = open(&tmp, c::O_PATH, 0).unwrap();
        openat(*dir, "a", c::O_CREAT | c::O_RDONLY, MODE).unwrap();
        metadata(format_ustr!("{}/a", tmp).as_path())
            .unwrap()
            .permissions()
            .mode()
            & 0o777
    };

    umask(0);
    assert_eq!(f(), MODE);
    umask(0o077);
    assert_eq!(f(), 0o700);
}
