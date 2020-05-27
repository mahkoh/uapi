use testutils::*;
use uapi::*;

#[test]
fn read_link() {
    let tmp = Tempdir::new();

    let path = &*format!("{}/a", tmp);

    let mut link = "x".to_string();
    for _ in 0..12 {
        link.push_str(&link.clone());
    }
    link.pop();

    symlink(&*link, path).unwrap();

    assert_eq!(&read_link_to_new_ustring(c::AT_FDCWD, path).unwrap(), &link);
    assert_eq!(
        &read_link_to_new_ustring(*open(tmp.bstr(), c::O_PATH, 0).unwrap(), "a").unwrap(),
        &link
    );

    let mut s = "xyz".to_string().into();
    assert_eq!(
        read_link_to_ustring(c::AT_FDCWD, path, &mut s).unwrap(),
        link.len()
    );

    assert_eq!(s, format!("xyz{}", link));
}
