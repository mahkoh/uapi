use std::process::exit;
use uapi::*;

#[test]
fn process3() {
    match unsafe { fork().unwrap() } {
        0 => exit(1),
        _ => {
            let (_, status) = wait().unwrap();
            assert_eq!(WIFEXITED(status), true);
            assert_eq!(WIFSIGNALED(status), false);
            assert_eq!(WIFSTOPPED(status), false);
            assert_eq!(WIFCONTINUED(status), false);
            assert_eq!(WEXITSTATUS(status), 1);
        }
    }
}
