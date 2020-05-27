use std::process::exit;
use testutils::*;
use uapi::*;

#[test]
fn process5() {
    match unsafe { fork().unwrap() } {
        0 => in_fork(|| {
            raise(c::SIGSTOP).unwrap();
            exit(1);
        }),
        n => {
            let (pid, status) = waitpid(n, c::WUNTRACED).unwrap();
            assert_eq!(pid, n);
            assert_eq!(WIFEXITED(status), false);
            assert_eq!(WIFSIGNALED(status), false);
            assert_eq!(WIFSTOPPED(status), true);
            assert_eq!(WIFCONTINUED(status), false);
            assert_eq!(WSTOPSIG(status), c::SIGSTOP);

            kill(n, c::SIGCONT).unwrap();

            let (pid, status) = waitpid(n, c::WCONTINUED).unwrap();
            assert_eq!(pid, n);
            assert_eq!(WIFEXITED(status), false);
            assert_eq!(WIFSIGNALED(status), false);
            assert_eq!(WIFSTOPPED(status), false);
            assert_eq!(WIFCONTINUED(status), true);

            let (pid, status) = waitpid(n, 0).unwrap();
            assert_eq!(pid, n);
            assert_eq!(WIFEXITED(status), true);
            assert_eq!(WIFSIGNALED(status), false);
            assert_eq!(WIFSTOPPED(status), false);
            assert_eq!(WIFCONTINUED(status), false);
            assert_eq!(WEXITSTATUS(status), 1);
        }
    }
}
