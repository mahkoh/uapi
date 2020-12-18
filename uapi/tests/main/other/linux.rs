use uapi::*;
use testutils::*;

#[test]
fn eventfd_() {
    let fd = eventfd(0, 0).unwrap();
    assert_ne!(fcntl_getfd(*fd).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let fd = eventfd(22, c::O_CLOEXEC).unwrap();
    assert_eq!(fcntl_getfd(*fd).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    eventfd_write(*fd, 11).unwrap();
    assert_eq!(eventfd_read(*fd).unwrap(), 33);

    eventfd_write(*fd, 11).unwrap();
    assert_eq!(eventfd_read(*fd).unwrap(), 11);

    let memfd = memfd_create("", 0).unwrap();
    write(*memfd, &[1]).unwrap();
    lseek(*memfd, 0, c::SEEK_SET).unwrap();

    assert_eq!(eventfd_read(*memfd).err().unwrap(), Errno(c::EBADF));
}

#[test]
fn memfd() {
    let fd = memfd_create("", 0).unwrap();
    assert_ne!(fcntl_getfd(*fd).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let fd = memfd_create("xyz", c::MFD_CLOEXEC as _).unwrap();
    assert_eq!(fcntl_getfd(*fd).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let name = read_link_to_new_ustring(0, format_ustr!("/proc/self/fd/{}", *fd)).unwrap();
    assert!(name.starts_with(b"/memfd:xyz"));

    write(*fd, &[1]).unwrap();
    lseek(*fd, 0, c::SEEK_SET).unwrap();
    let mut buf = [0];
    read(*fd, &mut buf[..]).unwrap();
    assert_eq!(buf[0], 1);
}

#[test]
fn syncfs_() {
    let tmp = Tempdir::new();

    assert!(syncfs(*open(&tmp, c::O_RDONLY, 0).unwrap()).is_ok());
    assert!(syncfs(-1).is_err());
}

#[test]
fn pipe2_() {
    let (r, _) = pipe2(0).unwrap();
    assert_ne!(fcntl_getfd(*r).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let (r, w) = pipe2(c::O_CLOEXEC).unwrap();
    assert_eq!(fcntl_getfd(*r).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    write(*w, &[1]).unwrap();
    let mut buf = [0];
    read(*r, &mut buf[..]).unwrap();
    assert_eq!(buf[0], 1);
}

#[test]
fn sysinfo_() {
    sysinfo().unwrap();
}
