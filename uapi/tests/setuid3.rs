use std::process::exit;
use testutils::*;
use uapi::*;

#[test]
fn setpid() {
    match unsafe { fork().unwrap() } {
        0 => in_fork(|| {
            assert_ne!(getsid(0), getpid());
            assert_ne!(getpgrp(), getpid());
            setsid().unwrap();
            assert_eq!(getsid(0), getpid());
            assert_eq!(getpgrp(), getpid());

            raise(c::SIGSTOP).unwrap();

            let c1 = match unsafe { fork().unwrap() } {
                0 => {
                    assert_eq!(getpgrp(), getppid());
                    assert_eq!(getsid(getppid()), getppid());
                    assert_eq!(getsid(0), getppid());
                    raise(c::SIGSTOP).unwrap();
                    assert_eq!(getpgrp(), getpid());
                    exit(0);
                }
                n => n,
            };

            let r1 = waitpid(c1, c::WUNTRACED).unwrap().1;
            assert!(WIFSTOPPED(r1));

            setpgid(c1, c1).unwrap();

            kill(c1, c::SIGCONT).unwrap();

            let r1 = waitpid(c1, 0).unwrap().1;
            assert!(WIFEXITED(r1));
            assert_eq!(WEXITSTATUS(r1), 0);
            exit(0);
        }),
        n => {
            let r1 = waitpid(n, c::WUNTRACED).unwrap().1;
            assert!(WIFSTOPPED(r1));

            assert_eq!(getsid(n), n);

            kill(n, c::SIGCONT).unwrap();

            let r1 = waitpid(n, 0).unwrap().1;
            assert!(WIFEXITED(r1));
            assert_eq!(WEXITSTATUS(r1), 0);
        }
    }
}
