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
