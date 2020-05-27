use std::process::exit;
use testutils::*;
use uapi::*;

#[test]
fn lock() {
    let tmp = Tempdir::new();

    let path = &*format!("{}/a", tmp);

    let fd = open(path, c::O_CREAT | c::O_RDWR, 0o777).unwrap();

    ftruncate(*fd, 1000).unwrap();

    let mut lk: c::flock = pod_zeroed();
    lk.l_type = c::F_WRLCK as _;
    lk.l_whence = c::SEEK_SET as _;
    lk.l_start = 3 as _;
    lk.l_len = 3 as _;

    fcntl_setlk(*fd, &lk).unwrap();

    match unsafe { fork().unwrap() } {
        0 => in_fork(|| {
            let mut lk2: c::flock = pod_zeroed();

            lk2.l_type = c::F_RDLCK as _;
            lk2.l_whence = c::SEEK_SET as _;
            lk2.l_start = 4 as _;
            lk2.l_len = 1 as _;

            fcntl_getlk(*fd, &mut lk2).unwrap();

            assert_eq!(lk2.l_type, lk.l_type);
            assert_eq!(lk2.l_whence, lk.l_whence);
            assert_eq!(lk2.l_start, lk.l_start);
            assert_eq!(lk2.l_len, lk.l_len);
            assert_eq!(lk2.l_pid, getppid());

            exit(0);
        }),
        n => {
            let (_, ws) = waitpid(n, 0).unwrap();
            assert!(WIFEXITED(ws));
            assert_eq!(WEXITSTATUS(ws), 0);
        }
    }
}
