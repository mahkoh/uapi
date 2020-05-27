use uapi::*;

#[test]
fn result() {
    let e = Errno(1);

    assert_eq!(format!("{}", e), "1");

    assert_eq!(std::io::Error::from(e).raw_os_error(), Some(1));

    set_errno(33);

    assert_eq!(Errno::default(), Errno(33));
}
