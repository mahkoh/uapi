use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[test]
fn poll_() {
    let (r, w) = pipe().unwrap();

    let mut fds = [c::pollfd {
        fd: *r,
        events: c::POLLIN,
        revents: 0,
    }];
    assert_eq!(poll(&mut fds, 0), Ok(0));

    write(*w, &[0]).unwrap();

    assert_eq!(poll(&mut fds, 0), Ok(1));

    assert_eq!(fds[0].revents, c::POLLIN);
}
