use uapi::*;
use std::mem;

#[test]
fn epoll() {
    let e = epoll_create1(0).unwrap();
    assert_ne!(fcntl_getfd(*e).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let e = epoll_create1(c::EPOLL_CLOEXEC).unwrap();
    assert_eq!(fcntl_getfd(*e).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let (r, w) = pipe().unwrap();

    epoll_ctl(*e, c::EPOLL_CTL_ADD, *r, Some(&c::epoll_event { events: c::EPOLLIN as _, u64: 3 })).unwrap();

    let mut events = unsafe { [mem::zeroed()] };

    assert_eq!(epoll_wait(*e, &mut events, 0).unwrap(), 0);

    write(*w, &[0]).unwrap();

    assert_eq!(epoll_wait(*e, &mut events, 1000).unwrap(), 1);

    let events = events[0].events;
    assert_eq!(events, c::EPOLLIN as _);
}
