use std::process::exit;
use testutils::*;
use uapi::*;

#[test]
fn process6() {
    match unsafe { fork().unwrap() } {
        0 => in_fork(|| {
            raise(c::SIGILL).unwrap();
            exit(1);
        }),
        n => {
            let (pid, status) = wait().unwrap();
            assert_eq!(pid, n);
            assert_eq!(WIFEXITED(status), false);
            assert_eq!(WIFSIGNALED(status), true);
            assert_eq!(WIFSTOPPED(status), false);
            assert_eq!(WIFCONTINUED(status), false);
            assert_eq!(WTERMSIG(status), c::SIGILL);
            #[cfg(not(target_os = "macos"))]
            assert_eq!(WCOREDUMP(status), true);
        }
    }
}
