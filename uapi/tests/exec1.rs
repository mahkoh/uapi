use uapi::*;

fn check_exit(n: c::pid_t, code: c::c_int) {
    let (pid, status) = waitpid(n, 0).unwrap();
    assert_eq!(pid, n);
    assert_eq!(WIFEXITED(status), true);
    assert_eq!(WEXITSTATUS(status), code);
}

fn sh() -> UstrPtr<'static> {
    ["sh", "-c"].iter().copied().collect()
}

#[test]
fn exec1() {
    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit $a");

            let mut env = UstrPtr::new();
            env.push("a=55");

            execve("/bin/sh", &buf, &env).unwrap();
        }
        n => check_exit(n, 55),
    }

    std::env::set_var("a", "99");

    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit $a");

            execv("/bin/sh", &buf).unwrap();
        }
        n => check_exit(n, 99),
    }

    std::env::set_var("a", "68");

    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit $a");

            execvp("sh", &buf).unwrap();
        }
        n => check_exit(n, 68),
    }

    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit 22");

            fexecve(
                *open("/bin/sh", c::O_RDONLY, 0).unwrap(),
                &buf,
                &UstrPtr::new(),
            ).unwrap();
        }
        n => check_exit(n, 22),
    }
}
