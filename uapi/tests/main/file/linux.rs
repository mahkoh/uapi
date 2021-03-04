use proc::*;
use std::{
    collections::HashSet,
    io::{IoSlice, IoSliceMut, Write},
};
use testutils::*;
use uapi::{c::open_how, *};

#[test]
fn read_write1() {
    let tmp = Tempdir::new();

    let path1 = &*format!("{}/a", tmp);

    let file = open(path1, c::O_CREAT | c::O_RDWR, 0o777).unwrap();

    fallocate(*file, 0, 1, 2).unwrap();

    let xstat = fstat(*file).unwrap();
    assert_eq!(xstat.st_size, 3);

    std::fs::write(path1, "abc").unwrap();

    fallocate(
        *file,
        c::FALLOC_FL_PUNCH_HOLE | c::FALLOC_FL_KEEP_SIZE,
        1,
        1,
    )
    .unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "a\0c");

    let f2 = dup(*file).unwrap();
    assert_eq!(fcntl_getfd(*f2).unwrap() & c::FD_CLOEXEC, 0);

    dup3(*file, *f2, c::O_CLOEXEC).unwrap();
    assert_eq!(fcntl_getfd(*f2).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    pwritev2(*file, &[IoSlice::new(b"x")][..], 1, 0).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "axc");

    pwritev2(*file, &[IoSlice::new(b"x")][..], 1, 0).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "axc");

    let mut buf = [0; 128];
    let mut iovec = [IoSliceMut::new(&mut buf)];
    let buf = preadv2(*file, &mut iovec[..], 2, 0).unwrap();
    assert_eq!(buf.iter().next(), Some(&b"c"[..]));

    assert!(statfs(path1).is_ok());
    assert!(fstatfs(*file).is_ok());
}

#[test_if(linux_4_16)]
fn read_write2() {
    let tmp = Tempdir::new();

    let path1 = &*format!("{}/a", tmp);

    let file = open(path1, c::O_CREAT | c::O_RDWR, 0o777).unwrap();

    pwritev2(*file, &[IoSlice::new(b"a")][..], 0, 0).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "a");

    pwritev2(*file, &[IoSlice::new(b"x")][..], 1, c::RWF_APPEND).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "ax");

    write(*file, b"y").unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "yx");

    pwritev2(*file, &[IoSlice::new(b"y")][..], -1, c::RWF_APPEND).unwrap();

    assert_eq!(std::fs::read_to_string(path1).unwrap(), "yxy");
}

#[test]
fn copy_file_range_() {
    let tmp = Tempdir::new();

    let path1 = &*format!("{}/a", tmp);
    let path2 = &*format!("{}/b", tmp);

    let mut f1 = open(path1, c::O_CREAT | c::O_RDWR, 0).unwrap();
    let mut f2 = open(path2, c::O_CREAT | c::O_RDWR, 0).unwrap();

    f1.write_all(b"hello world").unwrap();

    let n = copy_file_range(*f1, Some(&mut 3), *f2, Some(&mut 2), 5, 0).unwrap();
    assert_eq!(n, 5);

    let res = f2.read_to_new_ustring().unwrap();
    assert_eq!(&res, "\0\0lo wo");
}

#[test]
fn renameat_() {
    let tmp = Tempdir::new();

    let dir = open(tmp.bstr(), c::O_PATH, 0).unwrap();

    let path1 = "a";
    let path2 = "b";

    let f1 = openat(*dir, path1, c::O_CREAT | c::O_RDWR, 0).unwrap();

    renameat2(*dir, path1, *dir, path2, 0).unwrap();

    assert!(faccessat(*dir, path1, 0, 0).is_err());
    assert!(faccessat(*dir, path2, 0, 0).is_ok());

    assert_eq!(
        fstatat(*dir, path2, 0).unwrap().st_ino,
        fstat(*f1).unwrap().st_ino
    );

    let f2 = openat(*dir, path1, c::O_CREAT | c::O_RDWR, 0).unwrap();

    renameat2(*dir, path1, *dir, path2, c::RENAME_EXCHANGE).unwrap();

    assert_eq!(
        fstatat(*dir, path2, 0).unwrap().st_ino,
        fstat(*f2).unwrap().st_ino
    );
    assert_eq!(
        fstatat(*dir, path1, 0).unwrap().st_ino,
        fstat(*f1).unwrap().st_ino
    );
}

#[test]
fn splice_() {
    let (mut r, mut w) = pipe().unwrap();

    let mut fd = memfd_create("", 0).unwrap();

    w.write_all(b"abcdefghi").unwrap();

    splice(*r, None, *fd, Some(&mut 3), 3, 0).unwrap();

    drop(w);

    let res = r.read_to_new_ustring().unwrap();
    assert_eq!(&res, "defghi");

    let res = fd.read_to_new_ustring().unwrap();
    assert_eq!(&res, "\0\0\0abc");
}

#[test]
fn tee_() {
    let (mut r1, mut w1) = pipe().unwrap();
    let (mut r2, w2) = pipe().unwrap();

    w1.write_all(b"abcdefghi").unwrap();

    tee(*r1, *w2, 3, 0).unwrap();

    drop(w1);
    drop(w2);

    let res = r1.read_to_new_ustring().unwrap();
    assert_eq!(&res, "abcdefghi");

    let res = r2.read_to_new_ustring().unwrap();
    assert_eq!(&res, "abc");
}

#[test]
fn inotify() {
    let e = inotify_init1(0).unwrap();
    assert_ne!(fcntl_getfd(*e).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let e = inotify_init1(c::IN_CLOEXEC | c::IN_NONBLOCK).unwrap();
    assert_eq!(fcntl_getfd(*e).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let tmp = Tempdir::new();
    let mut buf = [0; 128];

    assert_eq!(
        inotify_read(*e, &mut buf[..]).err().unwrap(),
        Errno(c::EAGAIN)
    );

    let w = inotify_add_watch(*e, tmp.bstr(), c::IN_CREATE).unwrap();

    assert_eq!(
        inotify_read(*e, &mut buf[..]).err().unwrap(),
        Errno(c::EAGAIN)
    );

    let path1 = &*format!("{}/a", tmp);
    let path2 = &*format!("{}/b", tmp);
    let path3 = &*format!("{}/c", tmp);

    open(path1, c::O_CREAT | c::O_RDONLY, 0).unwrap();
    open(path2, c::O_CREAT | c::O_RDONLY, 0).unwrap();

    let mut names = HashSet::new();
    names.insert(ustr!("a"));
    names.insert(ustr!("b"));

    for ev in inotify_read(*e, &mut buf[..]).unwrap() {
        assert_eq!(ev.mask, c::IN_CREATE);
        assert_eq!(ev.wd, w);
        assert!(names.remove(ev.name().as_ustr()));
    }

    assert!(names.is_empty());

    inotify_rm_watch(*e, w).unwrap();

    let events: Vec<InotifyEvent> = inotify_read(*e, &mut buf[..])
        .unwrap()
        .into_iter()
        .collect();
    assert_eq!(events.len(), 1);

    assert_eq!(events[0].mask, c::IN_IGNORED);
    assert_eq!(events[0].wd, w);

    open(path3, c::O_CREAT | c::O_RDONLY, 0).unwrap();

    assert_eq!(
        inotify_read(*e, &mut buf[..]).err().unwrap(),
        Errno(c::EAGAIN)
    );
}

#[test]
fn sendfile_() {
    let mut m = memfd_create("", 0).unwrap();
    let (mut r2, w2) = pipe().unwrap();

    m.write_all(b"hello world").unwrap();

    sendfile(*w2, *m, Some(&mut 3), 5).unwrap();

    close(w2).unwrap();

    assert_eq!(&r2.read_to_new_ustring().unwrap(), "lo wo");
}

#[test]
fn makedev_() {
    assert_eq!(makedev(22222, 333333), 87962295914005);
    assert_eq!(major(makedev(22222, 333333)), 22222);
    assert_eq!(minor(makedev(22222, 333333)), 333333);
}

#[test_if(linux_5_6)]
fn openat2_() {
    let tmp = Tempdir::new();
    let dir = &*format!("{}/a", tmp);
    mkdir(dir, 0o777).unwrap();
    let dfd = open(dir, c::O_PATH, 0).unwrap();
    {
        let mut how: open_how = pod_zeroed();
        how.mode = 0o777;
        how.flags = (c::O_CREAT | c::O_WRONLY | c::O_CLOEXEC) as u64;
        let mut file = openat2(*dfd, "../b", &how).unwrap();
        assert_eq!(fcntl_getfd(*file).unwrap(), c::FD_CLOEXEC);
        write!(file, "abc").unwrap();
    }
    {
        let mut how: open_how = pod_zeroed();
        how.flags = c::O_RDONLY as u64;
        let file = openat2(*dfd, "../b", &how).unwrap();
        assert_eq!(fcntl_getfd(*file).unwrap(), 0);
        assert_eq!(read_file(file.into()), "abc");
    }
    {
        let mut how: open_how = pod_zeroed();
        how.flags = c::O_RDONLY as u64;
        how.resolve = c::RESOLVE_IN_ROOT;
        let err = openat2(*dfd, "../b", &how).unwrap_err();
        assert_eq!(err.0, c::ENOENT);
    }
}
