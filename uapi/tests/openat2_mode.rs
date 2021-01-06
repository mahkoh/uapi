extern crate proc; // https://github.com/rust-lang/rust/issues/64450

#[cfg(target_os = "linux")]
mod wrapper {
    use proc::test_if;

    #[test_if(linux_5_6)]
    fn openat2_() {
        use std::{fs::metadata, os::unix::fs::PermissionsExt};
        use testutils::*;
        use uapi::{c::open_how, *};

        const MODE: c::mode_t = 0o712;

        let f = || {
            let tmp = Tempdir::new();
            let dir = open(&tmp, c::O_RDONLY, 0).unwrap();
            let mut how: open_how = pod_zeroed();
            how.mode = MODE as _;
            how.flags = (c::O_CREAT | c::O_RDONLY) as u64;
            openat2(*dir, "a", &how).unwrap();
            metadata(format_ustr!("{}/a", tmp).as_path())
                .unwrap()
                .permissions()
                .mode()
                & 0o777
        };

        umask(0);
        assert_eq!(f() as c::mode_t, MODE);
        umask(0o077);
        assert_eq!(f(), 0o700);
    }
}
