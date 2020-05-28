use uapi::*;

#[test]
fn dupfd() {
    let (fd, _) = pipe().unwrap();

    let dup = fcntl_dupfd(*fd, 100).unwrap();
    assert_eq!(*dup, 100);
    assert_ne!(fcntl_getfd(*dup).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let dup = fcntl_dupfd_cloexec(*fd, 101).unwrap();
    assert_eq!(*dup, 101);
    assert_eq!(fcntl_getfd(*dup).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);
}
