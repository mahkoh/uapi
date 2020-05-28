#[cfg(not(target_os = "macos"))]
mod wrapper {
    use proc::*;
    use std::{fs::metadata, os::unix::fs::PermissionsExt};
    use testutils::*;
    use uapi::*;

    #[test_if(root)]
    fn mknodat1() {
        const MODE: c::mode_t = 0o712;
        const DEV: c::dev_t = 999;

        let f = || {
            let tmp = Tempdir::new();
            let fd = open(&tmp, c::O_RDONLY, 0).unwrap();
            let path = format_ustr!("{}/a", tmp);
            mknodat(*fd, "a", c::S_IFCHR | MODE, DEV).unwrap();
            let stat = stat(&path).unwrap();
            assert_eq!(stat.st_rdev, DEV);
            assert_eq!(stat.st_mode & c::S_IFMT, c::S_IFCHR);
            metadata(path.as_path()).unwrap().permissions().mode() & 0o777
        };

        umask(0);
        assert_eq!(f() as c::mode_t, MODE);
        umask(0o077);
        assert_eq!(f() as c::mode_t, 0o700);
    }
}
