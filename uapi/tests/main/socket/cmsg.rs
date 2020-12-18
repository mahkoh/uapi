use std::{mem, mem::MaybeUninit};
use uapi::*;

#[test]
fn test() {
    let mut buf = [MaybeUninit::uninit(); 1024];
    let hdr = pod_zeroed::<c::cmsghdr>();
    let mut written = 0;

    {
        let mut buf = &mut buf[..];

        written += cmsg_write(&mut buf, hdr, b"hello world").unwrap();
        written += cmsg_write(&mut buf, hdr, b"ayo hol up").unwrap();
    }

    let mut buf = unsafe { buf[..written].slice_assume_init_ref() };

    let (_, _, data1) = cmsg_read(&mut buf).unwrap();
    let (_, _, data2) = cmsg_read(&mut buf).unwrap();

    assert_eq!(data1, b"hello world");
    assert_eq!(data2, b"ayo hol up");
}

#[test]
fn invalid() {
    let mut hdr = pod_zeroed::<c::cmsghdr>();
    hdr.cmsg_len = -200i16 as _;

    assert_eq!(cmsg_read(&mut &[][..]).err().unwrap(), Errno(c::EINVAL));
    assert_eq!(
        cmsg_read(&mut &[0u8; mem::size_of::<c::cmsghdr>()][..])
            .err()
            .unwrap(),
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
