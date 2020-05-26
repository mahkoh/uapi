use std::process::exit;
use uapi::*;

#[test]
fn process1() {
    let ppid = getpid();

    match unsafe { fork().unwrap() } {
        0 => {
            assert_ne!(getpid(), ppid);
            assert_eq!(getppid(), ppid);
            exit(0);
        }
        n => {
            let (pid, status) = wait().unwrap();
            assert_eq!(n, pid);
            assert!(WIFEXITED(status), true);
            assert_eq!(WEXITSTATUS(status), 0);
        }
    }
}
