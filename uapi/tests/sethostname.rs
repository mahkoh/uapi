use proc::*;
use uapi::*;

#[cfg(target_os = "linux")]
#[test_if_root]
fn sethostname1() {
    unshare(c::CLONE_NEWUTS).unwrap();
    let name = b"hello world";
    sethostname(name).unwrap();
    assert_eq!(gethostname(&mut [0; 128][..]).unwrap().as_ustr(), &name[..]);
}
