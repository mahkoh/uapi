use std::process::exit;
use uapi::*;

#[test]
fn process4() {
    match unsafe { fork().unwrap() } {
        0 => {
            raise(c::SIGINT).unwrap();
            unreachable!();
        }
        n => {
            let (_, status) = wait().unwrap();
            assert_eq!(WIFEXITED(status), false);
            assert_eq!(WIFSIGNALED(status), true);
            assert_eq!(WIFSTOPPED(status), false);
            assert_eq!(WIFCONTINUED(status), false);
            assert_eq!(WTERMSIG(status), c::SIGINT);
        }
    }
}
