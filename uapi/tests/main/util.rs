use testutils::*;
use uapi::*;

#[test]
fn read_link() {
    let tmp = Tempdir::new();

    let path = &*format!("{}/a", tmp);
    let path2 = &*format!("{}/b", tmp);

    let mut link = "x".to_string();
    for _ in 0..5 {
        link.push_str(&link.clone());
    }
    link.pop();

    symlink(&*link, path).unwrap();
    open(path2, c::O_CREAT | c::O_RDONLY, 0).unwrap();

    assert_eq!(&read_link_to_new_ustring(c::AT_FDCWD, path).unwrap(), &link);
    assert_eq!(
        &read_link_to_new_ustring(*open(tmp.bstr(), c::O_RDONLY, 0).unwrap(), "a").unwrap(),
        &link
    );
    assert_eq!(
        read_link_to_new_ustring(c::AT_FDCWD, path2).err().unwrap(),
        Errno(c::EINVAL)
    );

    let mut s = "xyz".to_string().into();
    assert_eq!(
        read_link_to_ustring(c::AT_FDCWD, path, &mut s).unwrap(),
        link.len()
    );
    assert_eq!(
        read_link_to_ustring(c::AT_FDCWD, path2, &mut s)
            .err()
            .unwrap(),
        Errno(c::EINVAL)
    );

    assert_eq!(s, format!("xyz{}", link));
}
