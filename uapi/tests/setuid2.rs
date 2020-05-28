use proc::*;
use uapi::*;

#[test_if(root)]
#[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
fn setpid() {
    setresgid(3, 4, 5).unwrap();
    assert_eq!(getresgid().unwrap(), (3, 4, 5));

    setresuid(1, 2, 0).unwrap();
    assert_eq!(getresuid().unwrap(), (1, 2, 0));
}
