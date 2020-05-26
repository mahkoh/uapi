use proc::*;
use uapi::*;

#[test_if_root]
fn setpid() {
    setgid(2).unwrap();
    assert_eq!(getgid(), 2);

    seteuid(3).unwrap();
    assert_eq!(geteuid(), 3);

    seteuid(0).unwrap();
    assert_eq!(geteuid(), 0);

    setegid(4).unwrap();
    assert_eq!(getegid(), 4);

    setgroups(&[5, 6]).unwrap();
    let mut buf = [0; 128];
    assert_eq!(getgroups(&mut buf).unwrap(), &[5, 6]);

    setuid(1).unwrap();
    assert_eq!(getuid(), 1);
}
