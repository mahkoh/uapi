use cfg_if::cfg_if;
use proc::*;
use std::io::{IoSlice, IoSliceMut};
use testutils::*;
use uapi::*;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[test]
fn read_write() {
    let tmp = Tempdir::new();

    let path = format_ustr!("{}/a", tmp);

    let fd = open(&path, c::O_CREAT | c::O_WRONLY, 0o777).unwrap();
    let fd2 = openat(*open(&tmp, c::O_RDONLY, 0).unwrap(), "a", c::O_RDONLY, 0).unwrap();

    let output = b"hello world";
    assert_eq!(write(*fd, output).unwrap(), 11);

    let pos = lseek(*fd, 0, c::SEEK_CUR).unwrap();
    assert_eq!(pos, 11);

    assert_eq!(lseek(*fd, 0, c::SEEK_SET).unwrap(), 0);

    let xstat = fstat(*fd).unwrap();
    assert_eq!(xstat.st_ino, fstat(*fd2).unwrap().st_ino);
    assert_eq!(11, xstat.st_size);
    assert_eq!(stat(&path).unwrap().st_ino, xstat.st_ino);

    let mut buf = [0; 11];
    assert_eq!(read(*fd2, &mut buf).unwrap(), 11);

    assert_eq!(buf, *output);

    ftruncate(*fd, 0).unwrap();

    let xstat = fstat(*fd).unwrap();
    assert_eq!(0, xstat.st_size);

    writev(*fd, &[IoSlice::new(output)]).unwrap();

    let xstat = fstat(*fd).unwrap();
    assert_eq!(11, xstat.st_size);

    lseek(*fd, 0, c::SEEK_SET).unwrap();

    let testtesttest = b"testtesttest";
    assert_eq!(pwrite(*fd, testtesttest, 11).unwrap(), 12);

    lseek(*fd2, 0, c::SEEK_SET).unwrap();

    let mut buf1 = [0; 11];
    let mut buf2 = [0; 11];

    let rd = readv(
        *fd2,
        &mut [IoSliceMut::new(&mut buf1), IoSliceMut::new(&mut buf2)],
    )
    .unwrap();
    assert_eq!(rd, 22);

    assert_eq!(&buf1, output);
    assert_eq!(&buf2, b"testtesttes");

    assert_eq!(close(OwnedFd::new(-1)), Err(Errno(c::EBADF)));

    let fd3 = dup(*fd).unwrap();
    assert_eq!(fstat(*fd3).unwrap().st_ino, xstat.st_ino);
    assert_eq!(
        fstat(dup2(*fd, *fd3).unwrap()).unwrap().st_ino,
        xstat.st_ino
    );

    lseek(*fd2, 0, c::SEEK_SET).unwrap();

    let mut buf = [0; 12];

    assert_eq!(pread(*fd2, &mut buf, 11).unwrap(), 12);
    assert_eq!(&buf, testtesttest);
}

#[test]
#[cfg(not(target_os = "macos"))]
fn read_write2() {
    use std::io::Write;

    let tmp = Tempdir::new();

    let path = format_ustr!("{}/a", tmp);

    let output = b"hello world";
    let testtesttest = b"testtesttest";

    let mut fd = open(&path, c::O_CREAT | c::O_RDWR, 0o777).unwrap();
    fd.write_all(output).unwrap();
    fd.write_all(testtesttest).unwrap();

    let mut buf1 = [0; 10];
    let mut buf2 = [0; 12];

    assert_eq!(
        preadv(
            *fd,
            &mut [IoSliceMut::new(&mut buf1), IoSliceMut::new(&mut buf2)],
            1
        )
            .unwrap(),
        22
    );
    assert_eq!(&buf1, b"ello world");
    assert_eq!(&buf2, testtesttest);

    truncate(&path, 0).unwrap();

    assert_eq!(
        pwritev(*fd, &[IoSlice::new(output), IoSlice::new(testtesttest)], 10).unwrap(),
        23
    );

    let mut buf0 = [0; 10];
    let mut buf1 = [0; 11];
    let mut buf2 = [0; 12];

    assert_eq!(
        preadv(
            *fd,
            &mut [
                IoSliceMut::new(&mut buf0),
                IoSliceMut::new(&mut buf1),
                IoSliceMut::new(&mut buf2)
            ],
            0
        )
            .unwrap(),
        33
    );
    assert_eq!(buf0, [0; 10]);
    assert_eq!(&buf1, output);
    assert_eq!(&buf2, testtesttest);

    truncate(&path, 0).unwrap();

    let xstat = fstat(*fd).unwrap();
    assert_eq!(xstat.st_size, 0);

    posix_fallocate(*fd, 1, 1).unwrap();

    let xstat = fstat(*fd).unwrap();
    assert_eq!(xstat.st_size, 2);

    assert!(posix_fadvise(*fd, 0, 0, 0).is_ok());
}

#[test]
#[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
fn read_write3() {
    let tmp = Tempdir::new();

    let path = format_ustr!("{}/a", tmp);

    let fd = open(&path, c::O_CREAT | c::O_RDWR, 0o777).unwrap();

    posix_fallocate(*fd, 1, 1).unwrap();

    let xstat = fstat(*fd).unwrap();
    assert_eq!(xstat.st_size, 2);

    assert!(posix_fadvise(*fd, 0, 0, 0).is_ok());
}

#[test]
fn metadata1() {
    let tmp = Tempdir::new();

    let path = &*format!("{}/a", tmp);
    let path2 = &*format!("{}/b", tmp);
    let path3 = &*format!("{}/c", tmp);

    open(path, c::O_CREAT | c::O_RDONLY, 0o400).unwrap();
    assert!(access(path, c::R_OK).is_ok());
    assert!(access(path, c::X_OK).is_err());

    symlink(path, path2).unwrap();

    let mut buf = [0; 128];

    let size = readlink(path2, &mut buf).unwrap();
    assert_eq!(path.as_bytes(), &buf[..size]);

    assert!(access(path2, 0).is_ok());
    unlink(path2).unwrap();
    assert!(access(path2, 0).is_err());

    let tmpdir = open(&tmp, c::O_RDONLY, 0).unwrap();
    symlinkat(path, *tmpdir, "b").unwrap();

    let mut buf = [0; 128];

    let size = readlinkat(*tmpdir, "b", &mut buf).unwrap();
    assert_eq!(path.as_bytes(), &buf[..size]);

    let xstat = fstatat(*tmpdir, "b", c::AT_SYMLINK_NOFOLLOW).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFLNK);

    let xstat = stat(path2).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFREG);

    let xstat = lstat(path2).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFLNK);

    unlinkat(*tmpdir, path2, 0).unwrap();
    assert!(access(path2, 0).is_err());

    mkdir(path2, 0).unwrap();
    let xstat = stat(path2).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFDIR);

    assert!(unlinkat(*tmpdir, path2, 0).is_err());
    assert!(unlinkat(*tmpdir, path2, c::AT_REMOVEDIR).is_ok());
    assert!(access(path2, 0).is_err());

    mkdirat(*tmpdir, path2, 0o777).unwrap();
    let xstat = stat(path2).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFDIR);

    rename(path, path3).unwrap();
    assert!(access(path, 0).is_err());
    assert!(access(path3, 0).is_ok());

    let dir2 = open(path2, c::O_RDONLY, 0).unwrap();

    renameat(*tmpdir, path3, *dir2, "a").unwrap();
    assert!(faccessat(*tmpdir, "c", 0, 0).is_err());
    assert!(access(format!("{}/{}", path2, "a"), 0).is_ok());

    open(path, c::O_CREAT | c::O_RDONLY, 0).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_mode & !c::S_IFMT, 0);

    chmod(path, 0o400).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_mode & !c::S_IFMT, 0o400);

    let file = open(path, c::O_RDONLY, 0).unwrap();

    fchmod(*file, 0o200).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_mode & !c::S_IFMT, 0o200);

    fchmodat(*tmpdir, "a", 0o100, 0).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_mode & !c::S_IFMT, 0o100);

    chmod(path, 0o400).unwrap();

    assert!(flock(*file, c::LOCK_EX).is_ok());
    assert!(flock(
        *open(path, c::O_RDONLY, 0).unwrap(),
        c::LOCK_EX | c::LOCK_NB
    )
    .is_err());

    let timespec = [
        c::timespec {
            tv_sec: 1,
            tv_nsec: 2,
        },
        c::timespec {
            tv_sec: 3,
            tv_nsec: 4,
        },
    ];
    assert!(utimensat(*tmpdir, "a", &timespec, 0).is_ok());

    let xstat = stat(path).unwrap();

    assert_eq!(xstat.st_atime, 1);
    assert_eq!(xstat.st_atime_nsec, 2);

    assert_eq!(xstat.st_mtime, 3);
    assert_eq!(xstat.st_mtime_nsec, 4);

    let timespec = [
        c::timespec {
            tv_sec: 5,
            tv_nsec: 6,
        },
        c::timespec {
            tv_sec: 7,
            tv_nsec: 8,
        },
    ];
    assert!(futimens(*file, &timespec).is_ok());

    let xstat = stat(path).unwrap();

    assert_eq!(xstat.st_atime, 5);
    assert_eq!(xstat.st_atime_nsec, 6);

    assert_eq!(xstat.st_mtime, 7);
    assert_eq!(xstat.st_mtime_nsec, 8);

    let timeval = [
        c::timeval {
            tv_sec: 13,
            tv_usec: 14,
        },
        c::timeval {
            tv_sec: 15,
            tv_usec: 16,
        },
    ];
    assert!(futimes(*file, &timeval).is_ok());

    let xstat = stat(path).unwrap();

    assert_eq!(xstat.st_atime, 13);
    assert_eq!(xstat.st_atime_nsec, 14000);

    assert_eq!(xstat.st_mtime, 15);
    assert_eq!(xstat.st_mtime_nsec, 16000);

    link(path, path3).unwrap();

    let ystat = stat(path3).unwrap();
    assert_eq!(ystat.st_ino, xstat.st_ino);

    unlink(path3).unwrap();

    linkat(*tmpdir, "a", *tmpdir, "c", 0).unwrap();

    let ystat = stat(path3).unwrap();
    assert_eq!(ystat.st_ino, xstat.st_ino);

    unlink(path3).unwrap();

    mkfifo(path3, 0).unwrap();

    let xstat = stat(path3).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFIFO);

    assert_eq!(isatty(-1), Err(Errno(c::EBADF)));
    assert_eq!(isatty(*file), Err(Errno(c::ENOTTY)));

    // not sure how to test these
    assert!(statvfs(path).is_ok());
    assert!(fstatvfs(*file).is_ok());
    assert!(fsync(*file).is_ok());
    assert!(pathconf(path, c::_PC_LINK_MAX).is_ok());
    assert!(fpathconf(*file, c::_PC_LINK_MAX).is_ok());
}

#[test]
#[cfg(not(target_os = "macos"))]
fn metadata2() {
    let tmp = Tempdir::new();
    let path = &*format!("{}/a", tmp);
    let path3 = &*format!("{}/c", tmp);

    let tmpdir = open(tmp.bstr(), c::O_RDONLY, 0).unwrap();
    let file = open(path, c::O_CREAT | c::O_RDONLY, 0).unwrap();
    assert!(fdatasync(*file).is_ok());

    mkfifoat(*tmpdir, "c", 0).unwrap();

    let xstat = stat(path3).unwrap();
    assert_eq!(xstat.st_mode & c::S_IFMT, c::S_IFIFO);
}

#[test]
#[cfg(not(target_os = "openbsd"))]
fn lutimes1() {
    let tmp = Tempdir::new();
    let path = &*format!("{}/a", tmp);
    open(path, c::O_CREAT | c::O_RDONLY, 0).unwrap();

    let timeval = [
        c::timeval {
            tv_sec: 9,
            tv_usec: 10,
        },
        c::timeval {
            tv_sec: 11,
            tv_usec: 12,
        },
    ];
    assert!(lutimes(path, &timeval).is_ok());

    let xstat = stat(path).unwrap();

    assert_eq!(xstat.st_atime, 9);
    assert_eq!(xstat.st_atime_nsec, 10000);

    assert_eq!(xstat.st_mtime, 11);
    assert_eq!(xstat.st_mtime_nsec, 12000);
}

#[test_if(root)]
fn chown1() {
    let tmp = Tempdir::new();

    let path = &*format!("{}/a", tmp);
    let path2 = &*format!("{}/b", tmp);

    symlink(path, path2).unwrap();

    let fd = open(path, c::O_CREAT | c::O_WRONLY, 0).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_uid, 0);
    // fails on macos
    // assert_eq!(xstat.st_gid, 0);

    chown(path2, 1, 2).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_uid, 1);
    assert_eq!(xstat.st_gid, 2);

    fchown(*fd, 3, 4).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_uid, 3);
    assert_eq!(xstat.st_gid, 4);

    fchownat(*open(&tmp, c::O_RDONLY, 0).unwrap(), "b", 5, 6, 0).unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_uid, 5);
    assert_eq!(xstat.st_gid, 6);

    fchownat(
        *open(&tmp, c::O_RDONLY, 0).unwrap(),
        "b",
        7,
        8,
        c::AT_SYMLINK_NOFOLLOW,
    )
    .unwrap();

    let xstat = stat(path).unwrap();
    assert_eq!(xstat.st_uid, 5);
    assert_eq!(xstat.st_gid, 6);

    let xstat = lstat(path2).unwrap();
    assert_eq!(xstat.st_uid, 7);
    assert_eq!(xstat.st_gid, 8);

    lchown(path2, 9, 10).unwrap();

    let xstat = stat(path2).unwrap();
    assert_eq!(xstat.st_uid, 5);
    assert_eq!(xstat.st_gid, 6);

    let xstat = lstat(path2).unwrap();
    assert_eq!(xstat.st_uid, 9);
    assert_eq!(xstat.st_gid, 10);
}
