use crate::{einval, *};
use std::mem;

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
#[notest]
pub fn pod_read<T: Pod, U: ?Sized>(u: &U) -> Result<T> {
    let mut t = pod_zeroed();
    pod_write(u, &mut t)?;
    Ok(t)
}

/// Converts an initial port of `u` to `T`
///
/// The size of `u` must be equal to or larger than the size of `T`.
#[notest]
pub fn pod_read_init<T: Pod, U: ?Sized>(u: &U) -> Result<T> {
    let mut t = pod_zeroed();
    pod_write_init(u, &mut t)?;
    Ok(t)
}

/// Writes `u` to `t`
///
/// `u` and `t` must have the same size.
#[notest]
pub fn pod_write<T: Pod + ?Sized, U: ?Sized>(u: &U, t: &mut T) -> Result<()> {
    if mem::size_of_val(t) != mem::size_of_val(u) {
        einval()
    } else {
        pod_write_init(u, t)
    }
}

/// Writes an initial portion of `u` to `t`
///
/// The size of `u` must be equal to or larger than the size of `t`.
#[notest]
pub fn pod_write_init<T: Pod + ?Sized, U: ?Sized>(u: &U, t: &mut T) -> Result<()> {
    if mem::size_of_val(t) > mem::size_of_val(u) {
        einval()
    } else {
        unsafe {
            let dst = t as *mut _ as *mut u8;
            let src = u as *const _ as *const u8;
            std::ptr::copy_nonoverlapping(black_box_id(src), dst, mem::size_of_val(t));
        }
        Ok(())
    }
}

unsafe impl<T: Pod> Pod for [T] {
}

macro_rules! imp {
    ($($path:path)*) => {
        $(unsafe impl Pod for $path {})*
    }
}

imp! {
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
    c::sockaddr_nl
    c::sockaddr_alg
    c::sockaddr_ll
    c::sockaddr_vm

    c::msghdr
    c::cmsghdr

    c::ucred
}
