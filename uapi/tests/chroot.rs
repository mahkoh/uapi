use proc::*;
use testutils::*;
use uapi::*;

#[test_if(root)]
fn chroot1() {
    let tmp = Tempdir::new();

    let path = format!("{}/a", tmp);

    let fd = open(path, c::O_CREAT | c::O_RDONLY, 0).unwrap();

    chroot(tmp.bstr()).unwrap();
    chdir("/").unwrap();

    assert_eq!(fstat(*fd).unwrap().st_ino, stat("a").unwrap().st_ino);
}
