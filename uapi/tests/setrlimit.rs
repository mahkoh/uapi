extern crate proc; // https://github.com/rust-lang/rust/issues/64450

use uapi::*;

#[test]
fn setrlimit_() {
    let mut limit = uapi::getrlimit(c::RLIMIT_NOFILE as _).unwrap();
    assert!(limit.rlim_cur > 0);
    assert!(limit.rlim_cur <= limit.rlim_max);
    limit.rlim_cur -= 1;
    uapi::setrlimit(c::RLIMIT_NOFILE as _, &limit).unwrap();
    let new = uapi::getrlimit(c::RLIMIT_NOFILE as _).unwrap();
    assert_eq!(new.rlim_cur, limit.rlim_cur);
}
