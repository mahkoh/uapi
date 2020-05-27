use std::{fs::metadata, os::unix::fs::PermissionsExt};
use testutils::*;
use uapi::*;

#[test]
fn open1() {
    const MODE: c::mode_t = 0o712;

    let f = || {
        let tmp = Tempdir::new();
        let path = format_ustr!("{}/a", tmp);
        open(&path, c::O_CREAT | c::O_RDONLY, MODE).unwrap();
        metadata(path.as_path()).unwrap().permissions().mode() & 0o777
    };

    umask(0);
    assert_eq!(f() as c::mode_t, MODE);
    umask(0o077);
    assert_eq!(f(), 0o700);
}
