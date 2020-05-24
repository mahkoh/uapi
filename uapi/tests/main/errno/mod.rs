use std::io::Error;
use uapi::{get_errno, set_errno};

#[test]
fn set_errno1() {
    let cmp = |i| {
        set_errno(i);
        assert_eq!(i, get_errno());
        assert_eq!(Some(i), Error::last_os_error().raw_os_error());
    };
    cmp(1);
    cmp(2);
}
