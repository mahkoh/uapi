use uapi::*;

#[test]
fn test() {
    let mut buf = [0; 1024];
    let hdr = pod_zeroed::<c::cmsghdr>();

    {
        let mut buf = &mut buf[..];

        cmsg_write(&mut buf, hdr, b"hello world").unwrap();
        cmsg_write(&mut buf, hdr, b"ayo hol up").unwrap();
    }

    let mut buf = &buf[..];

    let (_, hdr2, data1) = cmsg_read(&mut buf).unwrap();
    let (_, hdr3, data2) = cmsg_read(&mut buf).unwrap();

    assert_eq!(data1, b"hello world");
    assert_eq!(data2, b"ayo hol up");
}

#[test]
fn invalid() {
    let mut hdr = pod_zeroed::<c::cmsghdr>();
    hdr.cmsg_len = -200i16 as _;

    assert_eq!(cmsg_read(&mut &[][..]).err().unwrap(), Errno(c::EINVAL));
    assert_eq!(
        cmsg_read(&mut as_bytes(&hdr)).err().unwrap(),
        Errno(c::EINVAL)
    );
    assert_eq!(
        cmsg_write(&mut &mut [][..], hdr, &[0u8]).err().unwrap(),
        Errno(c::EINVAL)
    );

    hdr.cmsg_len = -1i8 as _;
    assert_eq!(
        cmsg_write(&mut &mut [][..], hdr, &[0u8]).err().unwrap(),
        Errno(c::EINVAL)
    );
}
