use crate::*;
use std::{
    fs::File,
    io::{IoSlice, IoSliceMut, Read, Write},
    mem,
    net::{TcpListener, TcpStream, UdpSocket},
    ops::Deref,
    os::{
        fd::{AsFd, AsRawFd, BorrowedFd, RawFd},
        raw::c_int,
        unix::{
            io::{FromRawFd, IntoRawFd},
            net::{UnixDatagram, UnixListener, UnixStream},
        },
    },
    process::{ChildStderr, ChildStdin, ChildStdout, Stdio},
};

/// An owned file descriptor
///
/// Upon `Drop`, the contained file descriptor will be closed.
/// Errors from `close()` are ignored.
///
/// The contained file descriptor can be accessed via deref: `*self`.
///
/// This struct can be converted `From` and `Into` various `std` types.
///
/// This struct implements [`AsFd`], however, note that we do not enforce the requirements
/// of [`BorrowedFd`].
#[derive(Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct OwnedFd {
    raw: c_int,
}

impl OwnedFd {
    pub fn new(raw: c_int) -> OwnedFd {
        Self { raw }
    }

    /// Shortcut for `Fd::new(*self)`
    pub fn borrow(&self) -> Fd {
        Fd::new(self.raw)
    }

    /// Returns `*self` and does not run `Drop`
    pub fn unwrap(self) -> c_int {
        let raw = self.raw;
        mem::forget(self);
        raw
    }

    /// Returns `*self`
    pub fn raw(&self) -> c_int {
        self.raw
    }
}

impl From<std::os::fd::OwnedFd> for OwnedFd {
    fn from(value: std::os::fd::OwnedFd) -> Self {
        Self::new(value.into_raw_fd())
    }
}

impl From<OwnedFd> for std::os::fd::OwnedFd {
    fn from(value: OwnedFd) -> Self {
        unsafe { std::os::fd::OwnedFd::from_raw_fd(value.unwrap()) }
    }
}

impl AsFd for OwnedFd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        assert_ne!(self.raw, -1);
        unsafe { BorrowedFd::borrow_raw(self.raw) }
    }
}

impl AsRawFd for OwnedFd {
    fn as_raw_fd(&self) -> RawFd {
        self.raw
    }
}

impl FromRawFd for OwnedFd {
    unsafe fn from_raw_fd(fd: RawFd) -> Self {
        Self::new(fd)
    }
}

impl IntoRawFd for OwnedFd {
    fn into_raw_fd(self) -> RawFd {
        self.unwrap()
    }
}

impl Deref for OwnedFd {
    type Target = c_int;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

impl Drop for OwnedFd {
    fn drop(&mut self) {
        unsafe {
            c::close(self.raw);
        }
    }
}

macro_rules! from {
    ($ty:ident) => {
        impl From<$ty> for OwnedFd {
            fn from(x: $ty) -> Self {
                OwnedFd::new(IntoRawFd::into_raw_fd(x))
            }
        }
    };
}

macro_rules! to {
    ($ty:ident) => {
        impl From<OwnedFd> for $ty {
            fn from(fd: OwnedFd) -> Self {
                unsafe { <$ty as FromRawFd>::from_raw_fd(fd.unwrap()) }
            }
        }
    };
}

macro_rules! bi {
    ($ty:ident) => {
        from!($ty);
        to!($ty);
    };
}

bi!(File);
bi!(TcpListener);
bi!(TcpStream);
bi!(UdpSocket);
bi!(UnixDatagram);
bi!(UnixStream);
bi!(UnixListener);

to!(Stdio);

from!(ChildStderr);
from!(ChildStdin);
from!(ChildStdout);

/// A borrowed file descriptor
///
/// The contained file descriptor can be accessed via deref: `*self`.
///
/// This struct implements `Read` and `Write`.
///
/// This struct implements [`AsFd`], however, note that we do not enforce the requirements
/// of [`BorrowedFd`].
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
#[repr(transparent)]
pub struct Fd {
    raw: c_int,
}

impl Fd {
    /// Creates a new `Fd`
    pub fn new(raw: c_int) -> Fd {
        Fd { raw }
    }

    /// Returns `*self`
    pub fn raw(self) -> c_int {
        self.raw
    }
}

impl AsFd for Fd {
    fn as_fd(&self) -> BorrowedFd<'_> {
        assert_ne!(self.raw, -1);
        unsafe { BorrowedFd::borrow_raw(self.raw) }
    }
}

impl AsRawFd for Fd {
    fn as_raw_fd(&self) -> RawFd {
        self.raw
    }
}

impl Deref for Fd {
    type Target = c_int;

    fn deref(&self) -> &Self::Target {
        &self.raw
    }
}

macro_rules! impl_io {
    ($ty:ident) => {
        impl Read for $ty {
            fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
                Ok(read(self.raw, buf)?.len())
            }

            fn read_vectored(
                &mut self,
                bufs: &mut [IoSliceMut<'_>],
            ) -> std::io::Result<usize> {
                Ok(readv(self.raw, bufs)?.len())
            }
        }

        impl Write for $ty {
            fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
                Ok(write(self.raw, buf)?)
            }

            fn write_vectored(&mut self, bufs: &[IoSlice<'_>]) -> std::io::Result<usize> {
                Ok(writev(self.raw, bufs)?)
            }

            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
    };
}

impl_io!(Fd);
impl_io!(OwnedFd);

impl PartialEq<Fd> for OwnedFd {
    fn eq(&self, other: &Fd) -> bool {
        self.raw == other.raw
    }
}

impl PartialEq<OwnedFd> for Fd {
    fn eq(&self, other: &OwnedFd) -> bool {
        self.raw == other.raw
    }
}
