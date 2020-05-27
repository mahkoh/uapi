use crate::*;
use std::{marker::PhantomData, mem};

/// Marker trait for Pod types
///
/// This is not a general Pod type and only supposed to be used for interaction with this
/// library.
///
/// See also the library documentation.
///
/// # Safety
///
/// For all sized types `T: Pod`, transmuting any array of type `[u8; size_of::<T>()]` to
/// `T` must produce a valid value.
///
/// For all types `T: Pod`, overwriting the contents of `t: &mut T` with any array of type
/// `[u8; size_of_val(t)]` must produce a valid value.
pub unsafe trait Pod {}

/// Returns an instance of `T` whose object representation is `0` in all non-padding bytes
pub fn pod_zeroed<T: Pod>() -> T {
    unsafe { mem::zeroed() }
}

/// Converts `u` to `T`
///
/// `u` and `T` must have the same size.
pub fn pod_read<T: Pod, U: Packed + ?Sized>(u: &U) -> Result<T> {
    let mut t = pod_zeroed();
    pod_write(u, &mut t)?;
    Ok(t)
}

/// Converts `u` into an iterator of `T`
///
/// The size of `u` must be a multiple of the size of `T`
pub fn pod_iter<'a, T: Pod + 'a, U: Packed + ?Sized>(
    u: &'a U,
) -> Result<impl Iterator<Item = T> + 'a> {
    if mem::size_of::<T>() != 0 && mem::size_of_val(u) % mem::size_of::<T>() != 0 {
        einval()
    } else {
        Ok(Iter {
            buf: as_bytes(u),
            _pd: PhantomData,
        })
    }
}

struct Iter<'a, T> {
    buf: &'a [u8],
    _pd: PhantomData<fn() -> T>,
}

impl<'a, T: Pod> Iterator for Iter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf.is_empty() {
            None
        } else {
            let t = pod_read_init(self.buf).unwrap();
            self.buf = &self.buf[mem::size_of::<T>()..];
            Some(t)
        }
    }
}

/// Converts an initial port of `u` to `T`
///
/// The size of `u` must be equal to or larger than the size of `T`.
pub fn pod_read_init<T: Pod, U: Packed + ?Sized>(u: &U) -> Result<T> {
    let mut t = pod_zeroed();
    pod_write_common_prefix(u, &mut t)?;
    Ok(t)
}

/// Writes `u` to `t`
///
/// `u` and `t` must have the same size.
pub fn pod_write<T: Pod + ?Sized, U: Packed + ?Sized>(u: &U, t: &mut T) -> Result<()> {
    if mem::size_of_val(t) != mem::size_of_val(u) {
        einval()
    } else {
        pod_write_common_prefix(u, t)
    }
}

/// Writes an initial portion of `u` to `t`
///
/// The size of `u` must be equal to or larger than the size of `t`.
fn pod_write_common_prefix<T: Pod + ?Sized, U: Packed + ?Sized>(
    u: &U,
    t: &mut T,
) -> Result<()> {
    if mem::size_of_val(t) > mem::size_of_val(u) {
        einval()
    } else {
        unsafe {
            let dst = t as *mut _ as *mut u8;
            let src = u as *const _ as *const u8;
            std::ptr::copy_nonoverlapping(src, dst, mem::size_of_val(t));
        }
        Ok(())
    }
}

unsafe impl<T: Pod> Pod for [T] {
}

macro_rules! imp_pod {
    ($($path:path)*) => {
        $(unsafe impl Pod for $path {})*
    }
}

imp_pod! {
    u8
    u16
    u32
    u64
    u128
    usize
    i8
    i16
    i32
    i64
    i128
    isize

    c::sockaddr
    c::sockaddr_storage
    c::sockaddr_un
    c::sockaddr_in
    c::sockaddr_in6

    c::msghdr
    c::cmsghdr

    c::in_pktinfo
    c::in6_pktinfo

    c::sigset_t
    c::siginfo_t
    c::flock
    c::timespec
    c::timeval

    OwnedFd
    Fd
}

#[cfg(target_os = "linux")]
imp_pod! {
    c::sockaddr_alg
    c::sockaddr_nl
    c::sockaddr_ll
    c::sockaddr_vm
    c::signalfd_siginfo
}

#[cfg(not(target_os = "macos"))]
imp_pod! {
    c::ucred
}

/// Marker trait for types without padding
///
/// # Safety
///
/// Types that implement this must not have padding bytes
pub unsafe trait Packed {}

unsafe impl<T: Packed> Packed for [T] {
}

macro_rules! imp_packed {
    ($($path:path)*) => {
        $(unsafe impl Packed for $path {})*
    }
}

imp_packed! {
    u8
    u16
    u32
    u64
    u128
    usize
    i8
    i16
    i32
    i64
    i128
    isize

    OwnedFd
    Fd
}

/// Returns the object representation of `t`
pub fn as_bytes<T: Packed + ?Sized>(t: &T) -> &[u8] {
    unsafe {
        let ptr = t as *const _ as *const u8;
        std::slice::from_raw_parts(ptr, mem::size_of_val(t))
    }
}
