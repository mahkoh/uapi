use std::process::exit;
use uapi::*;

macro_rules! check_eq {
    ($a:expr, $b:expr) => {
        if $a != $b {
            exit(1);
        }
    };
}

macro_rules! check_ne {
    ($a:expr, $b:expr) => {
        if $a == $b {
            exit(1);
        }
    };
}

macro_rules! check {
    ($a:expr) => {
        if !$a {
            exit(1);
        }
    };
}

#[test]
fn setpid() {
    match unsafe { fork().unwrap() } {
        0 => {
            check_ne!(getsid(0), getpid());
            check_ne!(getpgrp(), getpid());
            setsid().unwrap();
            check_eq!(getsid(0), getpid());
            check_eq!(getpgrp(), getpid());

            raise(c::SIGSTOP).unwrap();

            let c1 = match unsafe { fork().unwrap() } {
                0 => {
                    check_eq!(getpgrp(), getppid());
                    check_eq!(getsid(getppid()), getppid());
                    check_eq!(getsid(0), getppid());
                    raise(c::SIGSTOP).unwrap();
                    check_eq!(getpgrp(), getpid());
                    exit(0);
                }
                n => n,
            };

            let r1 = waitpid(c1, c::WUNTRACED).unwrap().1;
            check!(WIFSTOPPED(r1));

            setpgid(c1, c1).unwrap();

            kill(c1, c::SIGCONT).unwrap();

            let r1 = waitpid(c1, 0).unwrap().1;
            check!(WIFEXITED(r1));
            check_eq!(WEXITSTATUS(r1), 0);
            exit(0);
        }
        n => {
            let r1 = waitpid(n, c::WUNTRACED).unwrap().1;
            check!(WIFSTOPPED(r1));

            assert_eq!(getsid(n), n);

            kill(n, c::SIGCONT).unwrap();

            let r1 = waitpid(n, 0).unwrap().1;
            assert!(WIFEXITED(r1));
            assert_eq!(WEXITSTATUS(r1), 0);
        }
    }
}
