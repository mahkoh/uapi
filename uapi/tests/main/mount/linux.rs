use proc::*;
use std::{ffi::CStr, fs, os::raw::c_int, path::Path};
use tempfile::{tempdir, tempdir_in};
use uapi::*;
use testutils::Tempdir;
use std::io::{BufReader, BufRead};
use std::fs::File;

#[test_if(root)]
fn mount_() {
    let tmp = Tempdir::new();
    let path = format!("{}", tmp.bstr().display());

    mount("dummy", tmp.bstr(), "tmpfs", 0, None).unwrap();

    for line in BufReader::new(File::open("/proc/self/mountinfo").unwrap()).lines() {
        let line = line.unwrap();
        if line.contains(&path) {
            let fields: Vec<_> = line.split(' ').collect();
            assert_eq!(fields[3], "/");
            assert_eq!(fields[4], path);
            assert_eq!(fields[8], "tmpfs");
        }
    }

    let _fd = open(format!("{}/a", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();

    assert!(umount2(tmp.bstr(), 0).is_err());

    assert!(umount2(tmp.bstr(), c::MNT_DETACH).is_ok());
}

fn tmpfs() -> OwnedFd {
    let fs = fsopen(ustr!("tmpfs"), 0).unwrap();
    fsconfig_cmd_create(*fs).unwrap();
    fsmount(*fs, 0, 0).unwrap()
}

fn with_mount_point<'a, P: Into<Option<&'a Path>>, T, F: FnOnce(&Path) -> T>(
    base: P,
    f: F,
) -> T {
    struct Umount<'a>(&'a Path);

    impl<'a> Drop for Umount<'a> {
        fn drop(&mut self) {
            let _ = umount2(self.0, c::MNT_DETACH);
        }
    }

    let dir = match base.into() {
        Some(base) => tempdir_in(base),
        _ => tempdir(),
    }
    .unwrap();
    let _umount = Umount(dir.path());
    f(dir.path())
}

fn with_private_mount<T, F: FnOnce(&Path) -> T>(f: F) -> T {
    with_mount_point(None, |path| {
        let fs = tmpfs();
        move_mount(
            *fs,
            Ustr::empty(),
            c::AT_FDCWD,
            path,
            c::MOVE_MOUNT_F_EMPTY_PATH,
        )
        .unwrap();
        mount(Ustr::empty(), path, Ustr::empty(), c::MS_PRIVATE, None).unwrap();
        f(path)
    })
}

fn create_file<'a>(dfd: c_int, p: impl IntoUstr<'a>) {
    openat(dfd, p, c::O_CREAT | c::O_RDONLY, 0).unwrap();
}

#[test_if(root, linux_5_2)]
fn move_mount1() {
    with_private_mount(|path| {
        with_mount_point(path, |p1| {
            with_mount_point(path, |p2| {
                const FILE: &str = "test";

                let mnt = tmpfs();
                create_file(*mnt, FILE);

                // move to p1
                move_mount(
                    *mnt,
                    Ustr::empty(),
                    c::AT_FDCWD,
                    p1,
                    c::MOVE_MOUNT_F_EMPTY_PATH,
                )
                .unwrap();
                assert!(p1.join(FILE).exists());

                // move from p1 to p2
                move_mount(
                    *mnt,
                    Ustr::empty(),
                    c::AT_FDCWD,
                    p2,
                    c::MOVE_MOUNT_F_EMPTY_PATH,
                )
                .unwrap();
                assert!(!p1.join(FILE).exists());
                assert!(p2.join(FILE).exists());

                // open tree at p2 and move to p1
                let mnt = open_tree(c::AT_FDCWD, p2, 0).unwrap();
                move_mount(
                    *mnt,
                    Ustr::empty(),
                    c::AT_FDCWD,
                    p1,
                    c::MOVE_MOUNT_F_EMPTY_PATH,
                )
                .unwrap();
                assert!(p1.join(FILE).exists());
                assert!(!p2.join(FILE).exists());

                // move from p2 to p1
                move_mount(c::AT_FDCWD, p1, c::AT_FDCWD, p2, 0).unwrap();
                assert!(!p1.join(FILE).exists());
                assert!(p2.join(FILE).exists());

                // clone tree at p2 and attach to p1
                let mnt = open_tree(c::AT_FDCWD, p2, c::OPEN_TREE_CLONE).unwrap();
                move_mount(
                    *mnt,
                    Ustr::empty(),
                    c::AT_FDCWD,
                    p1,
                    c::MOVE_MOUNT_F_EMPTY_PATH,
                )
                .unwrap();
                assert!(p1.join(FILE).exists());
                assert!(p2.join(FILE).exists());
            });
        });
    });
}

#[test_if(root, linux_5_2)]
fn fspick1() {
    with_mount_point(None, |p1| {
        let mnt = tmpfs();
        move_mount(
            *mnt,
            Ustr::empty(),
            c::AT_FDCWD,
            p1,
            c::MOVE_MOUNT_F_EMPTY_PATH,
        )
        .unwrap();

        fs::create_dir(p1.join("a")).unwrap();

        // make read-only
        let fs = fspick(c::AT_FDCWD, p1, 0).unwrap();
        fsconfig_set_flag(*fs, CStr::from_bytes_with_nul(b"ro\0").unwrap()).unwrap();
        fsconfig_cmd_reconfigure(*fs).unwrap();

        assert!(fs::create_dir(p1.join("b")).is_err());
    });
}

#[test_if(root, linux_5_2)]
fn fsmount1() {
    let fs = fsopen("proc", 0).unwrap();
    fsconfig_cmd_create(*fs).unwrap();
    let mnt = fsmount(*fs, 0, 0).unwrap();

    // open /proc/cpuinfo
    openat(*mnt, "cpuinfo", c::O_RDONLY, 0).unwrap();
}
