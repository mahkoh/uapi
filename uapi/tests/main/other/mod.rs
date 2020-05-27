use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[test]
fn uname_() {
    let res = uname().unwrap();
    #[cfg(target_os = "linux")]
    assert_eq!(res.sysname().as_ustr(), "Linux");
}

#[test]
fn sysconf_() {
    let res = sysconf(c::_SC_ARG_MAX).unwrap();
    assert!(res >= 4096);
}
