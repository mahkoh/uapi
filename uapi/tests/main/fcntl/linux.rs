use uapi::*;
use testutils::Tempdir;

#[test]
fn seal() {
    let fd = memfd_create("", c::MFD_ALLOW_SEALING).unwrap();

    assert_eq!(fcntl_get_seals(*fd).unwrap(), 0);

    fcntl_add_seals(*fd, c::F_SEAL_SEAL).unwrap();

    assert_eq!(fcntl_get_seals(*fd).unwrap(), c::F_SEAL_SEAL);
}

#[test]
fn ofd() {
    let tmp = Tempdir::new();

    let path = &*format!("{}/a", tmp);

    let fd = open(path, c::O_CREAT | c::O_RDWR, 0o777).unwrap();

    ftruncate(*fd, 1000).unwrap();

    let mut lk: c::flock = pod_zeroed();
    lk.l_type = c::F_WRLCK as _;
    lk.l_whence = c::SEEK_SET as _;
    lk.l_start = 3 as _;
    lk.l_len = 3 as _;

    fcntl_ofd_setlk(*fd, &lk).unwrap();

    let mut lk2: c::flock = pod_zeroed();
    lk2.l_type = c::F_RDLCK as _;
    lk2.l_whence = c::SEEK_SET as _;
    lk2.l_start = 4 as _;
    lk2.l_len = 1 as _;

    fcntl_ofd_getlk(*open(path, c::O_RDWR, 0).unwrap(), &mut lk2).unwrap();

    assert_eq!(lk2.l_type, lk.l_type);
    assert_eq!(lk2.l_whence, lk.l_whence);
    assert_eq!(lk2.l_start, lk.l_start);
    assert_eq!(lk2.l_len, lk.l_len);
    assert_eq!(lk2.l_pid, -1);
}