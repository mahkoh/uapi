use proc::*;
use uapi::*;

#[test_if_root]
#[cfg(target_os = "linux")]
fn setpid() {
    setresgid(3, 4, 5).unwrap();
    assert_eq!(getresgid().unwrap(), (3, 4, 5));

    setresuid(1, 2, 0).unwrap();
    assert_eq!(getresuid().unwrap(), (1, 2, 0));
}
