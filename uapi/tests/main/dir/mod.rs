use std::collections::HashSet;
use testutils::*;
use uapi::*;

fn readdir_x<F: Fn(&Tempdir) -> Result<Dir>>(f: F) {
    let tmp = Tempdir::new();

    let mut result = HashSet::new();
    result.extend(
        ["a", "b", "c"]
            .iter()
            .map(|v| Bstr::from_str(v).to_ustring()),
    );

    for d in &result {
        create_file(format_ustr!("{}/{}", tmp, d.display()));
    }

    result.insert(".".into_ustr().into_owned());
    result.insert("..".into_ustr().into_owned());

    let mut dir = f(&tmp).unwrap();
    let mut entries = HashSet::new();

    while let Some(entry) = readdir(&mut dir) {
        assert!(entries.insert(entry.unwrap().name().as_ustr().to_ustring()));
    }

    assert_eq!(result, entries);
}

#[test]
fn readdir1() {
    readdir_x(|t| opendir(t));
}

#[test]
fn fdopendir1() {
    readdir_x(|t| {
        let fd = open(t, c::O_RDONLY, 0).unwrap();
        fdopendir(fd)
    });
}

fn next(dir: &mut Dir) -> Ustring {
    readdir(dir).unwrap().unwrap().name().as_ustr().to_ustring()
}

#[test]
fn seekdir1() {
    let tmp = Tempdir::new();

    let mut dir = unsafe { Dir::from_ptr(opendir(&tmp).unwrap().unwrap()) };

    let pos = telldir(&mut dir);

    let name1 = next(&mut dir);

    seekdir(&mut dir, pos);

    let name2 = next(&mut dir);
    let name3 = next(&mut dir);

    assert_eq!(name1, name2);
    assert_ne!(name1, name3);
}

#[test]
fn rewinddir1() {
    let tmp = Tempdir::new();

    create_file(format_ustr!("{}/a", tmp));

    let mut dir = opendir(&tmp).unwrap();

    let name1 = next(&mut dir);
    let name2 = next(&mut dir);

    rewinddir(&mut dir);

    let name3 = next(&mut dir);
    let name4 = next(&mut dir);

    assert_eq!(name1, name3);
    assert_eq!(name2, name4);
}

#[test]
fn dirfd1() {
    let tmp = Tempdir::new();

    let fd = open(&tmp, c::O_RDONLY, 0).unwrap();
    let num = *fd;

    let mut dir = fdopendir(fd).unwrap();

    assert_eq!(num, dirfd(&mut dir));
}
