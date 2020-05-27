use testutils::*;
use uapi::*;
// use proc::*;

#[ignore] // better not remove this
// #[test_if_root]
#[test]
#[cfg(target_os = "linux")]
fn pivot_root_() {
    let tmp = Tempdir::new();

    let sub1 = &*format!("{}/a", tmp);
    let sub2 = &*format!("{}/a/b", tmp);

    mount("", tmp.bstr(), "tmpfs", 0, None).unwrap();
    defer!(|| umount2(tmp.bstr(), 0).unwrap());

    mount("", tmp.bstr(), Ustr::null(), c::MS_PRIVATE, None).unwrap();

    mkdir(sub1, 0).unwrap();

    mount("", sub1, "tmpfs", 0, None).unwrap();
    defer!(|| umount2(sub1, 0).unwrap());

    mkdir(sub2, 0).unwrap();

    pivot_root(sub1, sub2).unwrap();
}
