#[cfg(target_os = "linux")]
mod wrapper {
    use proc::*;
    use uapi::*;

    #[test_if(root)]
    fn unshare_() {
        let hn = "abc123abc";

        let old_ns = open("/proc/self/ns/uts", c::O_RDONLY, 0).unwrap();

        unshare(c::CLONE_NEWUTS).unwrap();

        assert_ne!(gethostname(&mut [0; 128][..]).unwrap().as_ustr(), hn);

        sethostname(hn.as_bytes()).unwrap();

        assert_eq!(gethostname(&mut [0; 128][..]).unwrap().as_ustr(), hn);

        assert!(setns(*old_ns, c::CLONE_NEWPID).is_err());

        setns(*old_ns, 0).unwrap();

        assert_ne!(gethostname(&mut [0; 128][..]).unwrap().as_ustr(), hn);
    }
}
