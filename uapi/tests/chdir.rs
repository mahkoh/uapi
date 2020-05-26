use testutils::*;
use uapi::*;

#[test]
fn chdir1() {
    let tmp = Tempdir::new();

    let mut buf = [0; 1024];
    let old = getcwd(&mut buf).unwrap();

    chdir(&tmp).unwrap();

    assert_eq!(tmp.bstr(), &*std::env::current_dir().unwrap());

    chdir(old).unwrap();

    assert_eq!(old.as_ustr(), &*std::env::current_dir().unwrap());

    fchdir(*open(&tmp, c::O_PATH, 0).unwrap()).unwrap();

    assert_eq!(tmp.bstr(), &*std::env::current_dir().unwrap());
}
