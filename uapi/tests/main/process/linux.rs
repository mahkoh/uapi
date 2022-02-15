use libc::O_NONBLOCK;
use proc::test_if;
use std::io::{Read, Write};
use uapi::*;

#[test]
fn gettid_() {
    let tid = read_link_to_new_ustring(c::AT_FDCWD, "/proc/thread-self").unwrap();

    assert_eq!(tid, format!("{}/task/{}", getpid(), gettid()));
}

#[test_if(linux_5_10)]
fn pidfd_open_nonblock() {
    let pidfd = pidfd_open(getpid(), 0).unwrap();
    assert!(fcntl_getfl(pidfd.raw()).unwrap() & O_NONBLOCK == 0);
    let pidfd = pidfd_open(getpid(), c::PIDFD_NONBLOCK).unwrap();
    assert!(fcntl_getfl(pidfd.raw()).unwrap() & O_NONBLOCK != 0);
    let (mut read, write) = pipe().unwrap();
    let mut write_clone = pidfd_getfd(pidfd.raw(), write.raw(), 0).unwrap();
    assert!(write_clone.raw() != write.raw());
    write!(write_clone, "ayo").unwrap();
    drop(write_clone);
    drop(write);
    let mut s = String::new();
    read.read_to_string(&mut s).unwrap();
    assert_eq!(s, "ayo");
}
