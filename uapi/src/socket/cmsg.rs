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

#[repr(C)]
union HdrBytes {
    hdr: c::cmsghdr,
    bytes: [u8; align(HDR_SIZE)],
}

/// Returns the number of bytes needed to store a cmsg with data-length `data_len`
///
/// See also the crate documentation.
pub const fn cmsg_space(data_len: usize) -> usize {
    align(HDR_SIZE) + align(data_len)
}

/// Reads a cmsg from a buffer
///
/// This function will
/// - advance the buffer by the used space
/// - return the used space, the cmsg header, and the data buffer
///
/// Returns an error if no message could be read from the buffer.
///
/// See also the crate documentation.
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
    if cmsg_len < align(HDR_SIZE) {
        return einval();
    }
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

/// Writes a cmsg to a buffer
///
/// This function will
/// - set `hdr.cmsg_len` to the correct value
/// - write `hdr` and `data` to the buffer
/// - advance the buffer by the used space
/// - return the used space
///
/// Returns an error if there is not enough space in the buffer.
///
/// See also the crate documentation.
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
