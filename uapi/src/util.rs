use crate::*;

#[man("Uses readlinkat(2) on (`fd`, `path`) and appends the result to `buf`")]
pub fn read_link_to_ustring<'a>(
    fd: c::c_int,
    path: impl IntoUstr<'a>,
    buf: &mut Ustring,
) -> Result<usize> {
    let path = path.into_ustr();
    let stat = fstatat(fd, &path, c::AT_SYMLINK_NOFOLLOW)?;
    if stat.st_mode & c::S_IFLNK == 0 {
        return Err(Errno(c::EINVAL));
    }
    let mut size = stat.st_size as usize + 1;
    if size == 1 {
        size = 128;
    }
    loop {
        buf.reserve_exact(size);
        let mut retry = false;
        let res = unsafe {
            buf.with_unused(|buf| {
                let buf_len = buf.len();
                match readlinkat(fd, &path, buf) {
                    Ok(n) if n.len() == buf_len => {
                        retry = true;
                        Err(Errno(c::ENAMETOOLONG))
                    }
                    r => r.map(|r| r.len()),
                }
            })
        };
        if retry && size < c::PATH_MAX as usize {
            size *= 2;
        } else {
            return res;
        }
    }
}

/// Shortcut for `read_link_to_ustring` with a new `Ustring`
pub fn read_link_to_new_ustring<'a>(
    fd: c::c_int,
    path: impl IntoUstr<'a>,
) -> Result<Ustring> {
    let mut s = Ustring::new();
    read_link_to_ustring(fd, path, &mut s).map(|_| s)
}

extern "C" {
    fn uapi_black_box(ptr: *const u8) -> *mut u8;
}

/// Does nothing
///
/// However:
///
/// 1. If the argument was derived from a mutable reference, the compiler cannot
///    assume anything about the value of the pointed-to object after the call.
///
/// This implementation currently works but should be replaced by a compiler intrinsic.
pub(crate) fn black_box<T: ?Sized>(ptr: *const T) {
    unsafe {
        uapi_black_box(ptr as *const _);
    }
}

/// Returns the argument
///
/// However:
///
/// 1. If the argument was derived from a mutable reference, the compiler cannot
///    assume anything about the value of the pointed-to object after the call.
/// 2. The compiler does not know anything about the origin of the returned pointer.
///
/// This implementation currently works but should be replaced by a compiler intrinsic.
pub(crate) fn black_box_id<T>(ptr: *const T) -> *mut T {
    unsafe { uapi_black_box(ptr as *const _) as *mut _ }
}

/// Returns `Err(Errno(c::EINVAL))`
pub(crate) const fn einval<T>() -> Result<T> {
    Err(Errno(c::EINVAL))
}

pub(crate) trait Integer: Copy {
    const MAX_VALUE: Self;
}

macro_rules! imv {
    ($t:ty) => {
        impl Integer for $t {
            const MAX_VALUE: Self = <$t>::max_value();
        }
    };
}

imv!(i8);
imv!(i16);
imv!(i32);
imv!(i64);
imv!(i128);
imv!(isize);
imv!(u8);
imv!(u16);
imv!(u32);
imv!(u64);
imv!(u128);
imv!(usize);
