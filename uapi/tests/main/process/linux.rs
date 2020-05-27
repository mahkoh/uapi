use uapi::*;

#[test]
fn gettid_() {
    let tid = read_link_to_new_ustring(c::AT_FDCWD, "/proc/thread-self").unwrap();

    assert_eq!(tid, format!("{}/task/{}", getpid(), gettid()));
}
