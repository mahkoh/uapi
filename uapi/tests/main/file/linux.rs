use std::io::{IoSlice};
use testutils::*;
use uapi::*;
use std::{mem};

#[test]
fn read_write1() {
    let tmp = Tempdir::new();

    let path1 = &*format!("{}/a", tmp);

    let file = open(path1, c::O_CREAT | c::O_RDWR, 0o777).unwrap();

    fallocate(*file, 0, 1, 2).unwrap();

    let xstat = fstat(*file).unwrap();
    assert_eq!(xstat.st_size, 3);

    std::fs::write(path1, "abc").unwrap();

    fallocate(*file, c::FALLOC_FL_PUNCH_HOLE | c::FALLOC_FL_KEEP_SIZE, 1, 1).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "a\0c");

    let f2 = dup(*file).unwrap();
    assert_eq!(fcntl_getfd(*f2).unwrap() & c::FD_CLOEXEC, 0);

    dup3(*file, *f2, c::O_CLOEXEC).unwrap().unwrap();
    assert_eq!(fcntl_getfd(*f2).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    pwritev2(*file, &[IoSlice::new(b"x")], 1, 0).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "axc");

    pwritev2(*file, &[IoSlice::new(b"x")], 1, 0).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "axc");

    pwritev2(*file, &[IoSlice::new(b"x")], 1, c::RWF_APPEND).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "axcx");

    write(*file, b"y").unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "yxcx");

    pwritev2(*file, &[IoSlice::new(b"y")], -1, c::RWF_APPEND).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "yxcxy");

    write(*file, b"z").unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "yxcxyz");
}

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