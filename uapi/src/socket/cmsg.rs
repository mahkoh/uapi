use crate::*;
use std::{convert::TryFrom, mem};

#[cfg(target_os = "linux")]
const ALIGN: usize = mem::size_of::<usize>() - 1;

#[cfg(any(target_os = "dragonfly", target_os = "macos", target_os = "ios"))]
const ALIGN: usize = 4 - 1;

#[cfg(all(
    target_os = "freebsd",
    any(
        target_arch = "aarch64",
        target_arch = "arm",
        target_arch = "powerpc64",
        target_arch = "x86",
        target_arch = "x86_64"
    )
))]
const ALIGN: usize = mem::size_of::<usize>() - 1;

const fn align(size: usize) -> usize {
    (size + ALIGN) & !ALIGN
}

const HDR_SIZE: usize = mem::size_of::<c::cmsghdr>();

union HdrBytes {
    hdr: c::cmsghdr,
    bytes: [u8; align(HDR_SIZE)],
}

pub const fn cmsg_space(data_len: usize) -> usize {
    align(HDR_SIZE) + align(data_len)
}

pub fn cmsg_read<'a>(buf: &mut &'a [u8]) -> Result<(usize, c::cmsghdr, &'a [u8])> {
    if buf.len() < align(HDR_SIZE) {
        return einval();
    }
    let mut hdr_bytes = HdrBytes {
        bytes: [0; align(HDR_SIZE)],
    };
    unsafe {
        hdr_bytes.bytes.copy_from_slice(&buf[..align(HDR_SIZE)]);
    }
    let hdr = unsafe { hdr_bytes.hdr };
    let cmsg_len = match usize::try_from(hdr.cmsg_len) {
        Ok(l) => l,
        _ => return einval(),
    };
    if usize::max_value() - cmsg_len < ALIGN {
        return einval();
    }
    let cmsg_space = align(cmsg_len);
    if buf.len() < cmsg_space {
        return einval();
    }
    let data = &buf[align(HDR_SIZE)..cmsg_len];
    *buf = &buf[cmsg_space..];
    Ok((cmsg_space, hdr, data))
}

/// Writes a new cmsg to a buffer
///
/// - `hdr.cmsg_len` will be initialized by this function
/// - `buf` will be advanced by the number of bytes used
/// - re
pub fn cmsg_write<T: ?Sized>(
    buf: &mut &mut [u8],
    mut hdr: c::cmsghdr,
    data: &T,
) -> Result<usize> {
    let data_size = mem::size_of_val(data);
    let cmsg_space = cmsg_space(data_size);
    if buf.len() < cmsg_space {
        return einval();
    }
    hdr.cmsg_len = align(HDR_SIZE) + data_size;
    let ptr = buf.as_mut_ptr();
    unsafe {
        ptr.copy_from_nonoverlapping(&hdr as *const _ as *const _, HDR_SIZE);
        ptr.add(align(HDR_SIZE))
            .copy_from_nonoverlapping(data as *const _ as *const _, data_size);
        black_box(ptr);
    }
    *buf = &mut mem::replace(buf, &mut [])[cmsg_space..];
    Ok(cmsg_space)
}

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
