#[cfg(target_os = "linux")]
mod wrapper {
    use proc::*;
    use uapi::*;

    #[test_if(root)]
    fn sethostname1() {

        unshare(c::CLONE_NEWUTS).unwrap();
        let name = b"hello world";
        sethostname(name).unwrap();
        assert_eq!(gethostname(&mut [0; 128][..]).unwrap().as_ustr(), &name[..]);
    }
}
