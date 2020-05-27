use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

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

#[test]
#[cfg(not(target_os = "macos"))]
fn pipesize() {
    let (r, _) = pipe().unwrap();

    let len = fcntl_getpipe_sz(*r).unwrap();
    fcntl_setpipe_sz(*r, len + 1).unwrap();
    let len2 = fcntl_getpipe_sz(*r).unwrap();
    assert!(len2 > len);
    fcntl_setpipe_sz(*r, len).unwrap();
    let len2 = fcntl_getpipe_sz(*r).unwrap();
    assert_eq!(len2, len);
}
