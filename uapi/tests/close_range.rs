extern crate proc; // https://github.com/rust-lang/rust/issues/64450

#[cfg(target_os = "linux")]
mod wrapper {
    use proc::test_if;
    use uapi::{c, close_range, fcntl_getfd, memfd_create, opendir, readdir, AsUstr};

    fn get_open_fds() -> Vec<c::c_int> {
        let mut fds = opendir("/proc/self/fd").unwrap();
        let mut res = vec![];
        while let Some(entry) = readdir(&mut fds) {
            let entry = entry.unwrap();
            let str = entry.name().as_ustr().as_str().unwrap();
            if str != "." && str != ".." {
                res.push(str.parse().unwrap());
            }
        }
        res
    }

    #[test_if(linux_5_9)]
    fn close_range_() {
        const LIMIT: c::c_int = 128;

        let memfd = memfd_create("dummy", 0).unwrap();
        assert!(get_open_fds().into_iter().all(|fd| fd < LIMIT));
        uapi::dup2(memfd.raw(), LIMIT).unwrap();
        assert!(get_open_fds().into_iter().any(|fd| fd >= LIMIT));
        close_range(LIMIT as _, c::c_uint::MAX, 0).unwrap();
        assert!(get_open_fds().into_iter().all(|fd| fd < LIMIT));
        uapi::dup2(memfd.raw(), LIMIT).unwrap();
        uapi::dup2(memfd.raw(), LIMIT + 10).unwrap();
        uapi::dup2(memfd.raw(), LIMIT + 20).unwrap();
        assert_eq!(fcntl_getfd(LIMIT).unwrap() & c::FD_CLOEXEC, 0);
        assert_eq!(fcntl_getfd(LIMIT + 10).unwrap() & c::FD_CLOEXEC, 0);
        assert_eq!(fcntl_getfd(LIMIT + 20).unwrap() & c::FD_CLOEXEC, 0);
        close_range(LIMIT as _, (LIMIT + 15) as _, c::CLOSE_RANGE_CLOEXEC).unwrap();
        assert_ne!(fcntl_getfd(LIMIT).unwrap() & c::FD_CLOEXEC, 0);
        assert_ne!(fcntl_getfd(LIMIT + 10).unwrap() & c::FD_CLOEXEC, 0);
        assert_eq!(fcntl_getfd(LIMIT + 20).unwrap() & c::FD_CLOEXEC, 0);
    }
}
