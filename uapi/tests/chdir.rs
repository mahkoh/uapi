use testutils::*;
use uapi::*;

#[test]
fn chdir1() {
    let tmp = Tempdir::new();
    let tmpdir = &std::fs::canonicalize(tmp.bstr()).unwrap();

    let mut buf1 = [0; 1024];
    let mut buf2 = [0; 1024];
    let old = getcwd(&mut buf1).unwrap();

    chdir(&tmp).unwrap();

    assert_eq!(tmpdir, &*std::env::current_dir().unwrap());

    chdir(old).unwrap();

    assert_eq!(old.as_ustr(), &*std::env::current_dir().unwrap());

    fchdir(*open(&tmp, c::O_RDONLY, 0).unwrap()).unwrap();

    assert_eq!(tmpdir, &*std::env::current_dir().unwrap());
}
