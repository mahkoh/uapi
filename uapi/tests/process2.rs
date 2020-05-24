use std::process::exit;
use testutils::*;
use uapi::*;

#[test]
fn process2() {
    match unsafe { fork().unwrap() } {
        0 => in_fork(|| exit(0)),
        _ => {
            let (_, status) = wait().unwrap();
            assert_eq!(WIFEXITED(status), true);
            assert_eq!(WIFSIGNALED(status), false);
            assert_eq!(WIFSTOPPED(status), false);
            assert_eq!(WIFCONTINUED(status), false);
            assert_eq!(WEXITSTATUS(status), 0);
        }
    }
}
