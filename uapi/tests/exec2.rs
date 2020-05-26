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
#[cfg(target_os = "linux")]
fn exec2() {
    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit $a");

            let mut env = UstrPtr::new();
            env.push("a=55");

            execveat(*open("/bin", c::O_RDONLY, 0).unwrap(), "sh", &buf, &env, 0).unwrap();
        }
        n => check_exit(n, 55),
    }

    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit $a");

            let mut env = UstrPtr::new();
            env.push("a=56");

            execveat(
                *open("/bin/sh", c::O_RDONLY, 0).unwrap(),
                "",
                &buf,
                &env,
                c::AT_EMPTY_PATH,
            ).unwrap();
        }
        n => check_exit(n, 56),
    }
    match unsafe { fork().unwrap() } {
        0 => {
            let mut buf = sh();
            buf.push("exit $a");

            let mut env = UstrPtr::new();
            env.push("a=57");

            execvpe("sh", &buf, &env).unwrap();
        }
        n => check_exit(n, 57),
    }
}
