use uapi::*;

#[test]
fn setfl() {
    let (r, _w) = pipe().unwrap();
    let old = fcntl_getfl(*r).unwrap();
    assert_eq!(old & c::O_NONBLOCK, 0);
    fcntl_setfl(*r, old | c::O_NONBLOCK).unwrap();
    assert_eq!(read(*r, &mut [0]), Err(Errno(c::EAGAIN)));
    assert_ne!(fcntl_getfl(*r).unwrap() & c::O_NONBLOCK, 0);
}

#[test]
fn setfd() {
    let (r, _w) = pipe().unwrap();
    let old = fcntl_getfd(*r).unwrap();
    assert_eq!(old & c::FD_CLOEXEC, 0);
    fcntl_setfd(*r, old | c::FD_CLOEXEC).unwrap();
    assert_ne!(fcntl_getfd(*r).unwrap() & c::FD_CLOEXEC, 0);
}
