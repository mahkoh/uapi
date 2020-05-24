use std::io::IoSliceMut;
use testutils::Tempdir;
use uapi::*;

fn tmp_file() -> (Tempdir, Ustring, &'static str) {
    let tmp = Tempdir::new();
    let path = format_ustr!("{}/a", tmp);
    let expected = "hello world";
    std::fs::write(&path, expected).unwrap();
    (tmp, path, expected)
}

#[test]
fn open1() {
    let (tmp, _, expected) = tmp_file();
    let path = format_ustr!("{}/b", tmp);
    {
        let fd = open(&path, c::O_CREAT | c::O_WRONLY, 0o711).unwrap();
        assert_eq!(expected.len(), write(*fd, expected.as_bytes()).unwrap());
    }
    let actual = open(&path, c::O_RDONLY, 0)
        .unwrap()
        .read_to_new_ustring()
        .unwrap();
    assert_eq!(&actual, expected);
}

#[test]
fn openat1() {
    let tmp = Tempdir::new();
    let file = "a";
    let path = format_ustr!("{}/{}", tmp, file);
    let expected = "hello world";
    std::fs::write(&path, expected).unwrap();
    let dir = open(&tmp, c::O_PATH, 0).unwrap();
    let actual = openat(*dir, file, c::O_RDONLY, 0)
        .unwrap()
        .read_to_new_ustring()
        .unwrap();
    assert_eq!(&actual, expected);
}

#[test]
fn read1() {
    let (_tmp, path, expected) = tmp_file();
    let mut buf = [0u8; 32];
    let n = read(*open(&path, c::O_RDONLY, 0).unwrap(), &mut buf).unwrap();
    assert!(n > 0);
    assert!(n <= buf.len());
    assert!(expected.as_bytes().starts_with(&buf[..n]));
}

#[test]
fn readv1() {
    let (_tmp, path, _) = tmp_file();
    let mut buf1 = [0u8];
    let mut buf2 = buf1;
    let n = {
        let mut bufs = [IoSliceMut::new(&mut buf1), IoSliceMut::new(&mut buf2)];
        readv(*open(&path, c::O_RDONLY, 0).unwrap(), &mut bufs).unwrap()
    };
    assert_eq!(n, 2);
    assert_eq!(buf1, [b'h']);
    assert_eq!(buf2, [b'e']);
}
