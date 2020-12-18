use std::{
    io::{IoSlice, IoSliceMut},
    mem,
    mem::MaybeUninit,
    slice,
};

/// A possibly uninitialized `iovec`
pub trait MaybeUninitIovec {
    /// Returns the `iovec`
    fn as_iovec(&self) -> &[&[MaybeUninit<u8>]];
}

/// A possibly uninitialized `iovec` (mutable)
pub trait MaybeUninitIovecMut {
    /// Returns the `iovec`
    ///
    /// # Safety
    ///
    /// The returnved `iovec` must not be used to write any uninitialized values.
    unsafe fn as_iovec_mut(&mut self) -> &mut [&mut [MaybeUninit<u8>]];
}

macro_rules! impl_maybe_uninit_iovec {
    ($($ty:ty)*) => {
        $(
            impl MaybeUninitIovec for [$ty] {
                fn as_iovec(&self) -> &[&[MaybeUninit<u8>]] {
                    unsafe { mem::transmute(self) }
                }
            }

            // TODO: https://github.com/rust-lang/rust/pull/79135
            // impl<const N: usize> MaybeUninitIovec for [$ty; N] {
            //     fn as_iovec(&self) -> &[&[MaybeUninit<u8>]] {
            //         unsafe { mem::transmute(&self[..]) }
            //     }
            // }
        )*
    }
}

impl_maybe_uninit_iovec! {
    IoSlice<'_>
    IoSliceMut<'_>
    &[u8]
    &mut [u8]
    &[MaybeUninit<u8>]
    &mut [MaybeUninit<u8>]
    &[i8]
    &mut [i8]
    &[MaybeUninit<i8>]
    &mut [MaybeUninit<i8>]
}

macro_rules! impl_maybe_uninit_iovec_mut {
    ($($ty:ty)*) => {
        $(
            impl MaybeUninitIovecMut for [$ty] {
                unsafe fn as_iovec_mut(&mut self) -> &mut [&mut [MaybeUninit<u8>]] {
                    mem::transmute(self)
                }
            }

            // TODO: https://github.com/rust-lang/rust/pull/79135
            // impl<const N: usize> MaybeUninitIovecMut for [$ty; N] {
            //     unsafe fn as_iovec_mut(&mut self) -> &mut [&mut [MaybeUninit<u8>]] {
            //         unsafe { mem::transmute(&mut self[..]) }
            //     }
            // }
        )*
    }
}

impl_maybe_uninit_iovec_mut! {
    IoSlice<'_>
    IoSliceMut<'_>
    &mut [u8]
    &mut [MaybeUninit<u8>]
    &mut [i8]
    &mut [MaybeUninit<i8>]
}

/// A wrapper for a partially initialized `iovec`
pub struct InitializedIovec<'a> {
    inner: InitializedIovecIter<'a>,
}

impl<'a> InitializedIovec<'a> {
    /// Returns an iterator over the initialized components of the `iovec`
    pub fn iter(&self) -> impl Iterator<Item = &'a [u8]> {
        self.inner.clone()
    }

    /// Returns the number of initialized bytes
    pub fn len(&self) -> usize {
        self.inner.initialized
    }

    /// Returns if there are no initialized bytes
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub(crate) unsafe fn new(
        buf: &'a [&'a mut [MaybeUninit<u8>]],
        initialized: usize,
    ) -> Self {
        Self {
            inner: InitializedIovecIter {
                buf: mem::transmute(buf),
                initialized,
            },
        }
    }
}

impl<'a> IntoIterator for InitializedIovec<'a> {
    type Item = &'a [u8];
    type IntoIter = InitializedIovecIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner
    }
}

/// An iterator over the initialized components of an `iovec`
#[derive(Clone)]
pub struct InitializedIovecIter<'a> {
    buf: &'a [&'a [MaybeUninit<u8>]],
    initialized: usize,
}

impl<'a> Iterator for InitializedIovecIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.initialized == 0 {
            return None;
        }
        let buf = self.buf[0];
        let len = self.initialized.min(buf.len());
        self.initialized -= len;
        self.buf = &self.buf[1..];
        unsafe { Some(buf[..len].slice_assume_init_ref()) }
    }
}

/// Returns the object representation of `t`
pub fn as_maybe_uninit_bytes<T: ?Sized>(t: &T) -> &[MaybeUninit<u8>] {
    unsafe {
        let ptr = t as *const _ as *const MaybeUninit<u8>;
        slice::from_raw_parts(ptr, mem::size_of_val(t))
    }
}

/// Returns the mutable object representation of `t`
pub fn as_maybe_uninit_bytes_mut<T>(t: &mut MaybeUninit<T>) -> &mut [MaybeUninit<u8>] {
    unsafe {
        let ptr = t as *mut _ as *mut MaybeUninit<u8>;
        slice::from_raw_parts_mut(ptr, mem::size_of_val(t))
    }
}

/// Returns the mutable object representation of `t`
///
/// This function exists because we cannot call as_maybe_uninit_mut for unsized T.
///
/// # Safety
///
/// The returned reference must not be used to write uninitialized data into `t`.
pub(crate) unsafe fn as_maybe_uninit_bytes_mut2<T: ?Sized>(
    t: &mut T,
) -> &mut [MaybeUninit<u8>] {
    let ptr = t as *mut _ as *mut MaybeUninit<u8>;
    slice::from_raw_parts_mut(ptr, mem::size_of_val(t))
}

/// Casts the argument to `MaybeUninit` of the same type
pub fn as_maybe_uninit<T>(t: &T) -> &MaybeUninit<T> {
    unsafe { mem::transmute(t) }
}

/// Casts the argument to `MaybeUninit` of the same type
///
/// # Safety
///
/// The returned reference must not be used to write uninitialized data into `t`.
pub unsafe fn as_maybe_uninit_mut<T>(t: &mut T) -> &mut MaybeUninit<T> {
    mem::transmute(t)
}

mod sealed {
    pub trait Sealed {}
}

/// Extension for [`MaybeUninit`]
#[allow(clippy::missing_safety_doc)]
pub trait MaybeUninitSliceExt<T>: sealed::Sealed {
    /// [`MaybeUninit::slice_assume_init_ref`]
    unsafe fn slice_assume_init_ref(&self) -> &[T];
    /// [`MaybeUninit::slice_assume_init_mut`]
    unsafe fn slice_assume_init_mut(&mut self) -> &mut [T];
}

impl<T> sealed::Sealed for [MaybeUninit<T>] {
}

impl<T> MaybeUninitSliceExt<T> for [MaybeUninit<T>] {
    unsafe fn slice_assume_init_ref(&self) -> &[T] {
        mem::transmute(self)
    }

    unsafe fn slice_assume_init_mut(&mut self) -> &mut [T] {
        mem::transmute(self)
    }
}

#[cfg(test)]
mod test {
    use crate::c;
    use std::{io::IoSlice, mem};

    #[test]
    fn iovec_repr() {
        let buf = [0u8; 11];

        let slice = &buf[..];
        let iovec = IoSlice::new(slice);

        unsafe {
            assert_eq!(mem::size_of_val(&iovec), mem::size_of_val(&slice));
            assert_eq!(
                c::memcmp(
                    &iovec as *const _ as *const _,
                    &slice as *const _ as *const _,
                    mem::size_of_val(&slice)
                ),
                0
            );
        }
    }
}
