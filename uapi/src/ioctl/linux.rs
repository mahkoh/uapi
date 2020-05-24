#![allow(non_snake_case)]

use cfg_if::cfg_if;
use std::mem;

cfg_if! {
    if #[cfg(any(target_arch = "mips", target_arch = "mips64", target_arch = "powerpc", target_arch = "powerpc64", target_arch = "sparc64"))] {
        /// [`_IOC_NONE`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_NONE: u64 = 1;
        /// [`_IOC_READ`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_READ: u64 = 2;
        /// [`_IOC_WRITE`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_WRITE: u64 = 4;
        /// [`_IOC_SIZEBITS`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_SIZEBITS: u64 = 13;
        /// [`_IOC_DIRBITS`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_DIRBITS: u64 = 3;
    }
}
cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "arm", target_arch = "s390x", target_arch = "x86_64", target_arch = "aarch64", target_arch = "riscv64"))] {
        /// [`_IOC_NONE`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_NONE: u64 = 0;
        /// [`_IOC_READ`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_READ: u64 = 2;
        /// [`_IOC_WRITE`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_WRITE: u64 = 1;
        /// [`_IOC_SIZEBITS`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_SIZEBITS: u64 = 14;
        /// [`_IOC_DIRBITS`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
        pub const _IOC_DIRBITS: u64 = 2;
    }
}

/// [`_IOC_NRBITS`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_NRBITS: u64 = 8;
/// [`_IOC_TYPEBITS`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_TYPEBITS: u64 = 8;
/// [`_IOC_NRSHIFT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_NRSHIFT: u64 = 0;
/// [`_IOC_TYPESHIFT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_TYPESHIFT: u64 = _IOC_NRSHIFT + _IOC_NRBITS as u64;
/// [`_IOC_SIZESHIFT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_SIZESHIFT: u64 = _IOC_TYPESHIFT + _IOC_TYPEBITS as u64;
/// [`_IOC_DIRSHIFT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_DIRSHIFT: u64 = _IOC_SIZESHIFT + _IOC_SIZEBITS as u64;
/// [`_IOC_NRMASK`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_NRMASK: u64 = (1 << _IOC_NRBITS) - 1;
/// [`_IOC_TYPEMASK`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_TYPEMASK: u64 = (1 << _IOC_TYPEBITS) - 1;
/// [`_IOC_SIZEMASK`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_SIZEMASK: u64 = (1 << _IOC_SIZEBITS) - 1;
/// [`_IOC_DIRMASK`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const _IOC_DIRMASK: u64 = (1 << _IOC_DIRBITS) - 1;

/// [`_IOC`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOC(dir: u64, ty: u64, nr: u64, size: u64) -> u64 {
    (dir << _IOC_DIRSHIFT)
        | (ty << _IOC_TYPESHIFT)
        | (nr << _IOC_NRSHIFT)
        | (size << _IOC_SIZESHIFT)
}

/// [`_IO`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IO(ty: u64, nr: u64) -> u64 {
    _IOC(_IOC_NONE, ty, nr, 0)
}

/// [`_IOR`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOR<T>(ty: u64, nr: u64) -> u64 {
    _IOC(_IOC_READ, ty, nr, mem::size_of::<T>() as _)
}

/// [`_IOW`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOW<T>(ty: u64, nr: u64) -> u64 {
    _IOC(_IOC_WRITE, ty, nr, mem::size_of::<T>() as _)
}

/// [`_IOWR`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOWR<T>(ty: u64, nr: u64) -> u64 {
    _IOC(_IOC_READ | _IOC_WRITE, ty, nr, mem::size_of::<T>() as _)
}

/// [`_IOC_DIR`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOC_DIR(nr: u64) -> u64 {
    (nr >> _IOC_DIRSHIFT) & _IOC_DIRMASK
}

/// [`_IOC_TYPE`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOC_TYPE(nr: u64) -> u64 {
    (nr >> _IOC_TYPESHIFT) & _IOC_TYPEMASK
}

/// [`_IOC_NR`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOC_NR(nr: u64) -> u64 {
    (nr >> _IOC_NRSHIFT) & _IOC_NRMASK
}

/// [`_IOC_SIZE`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const fn _IOC_SIZE(nr: u64) -> u64 {
    (nr >> _IOC_SIZESHIFT) & _IOC_SIZEMASK
}

/// [`IOC_IN`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const IOC_IN: u64 = _IOC_WRITE << _IOC_DIRSHIFT;

/// [`IOC_OUT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const IOC_OUT: u64 = _IOC_READ << _IOC_DIRSHIFT;

/// [`IOC_INOUT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const IOC_INOUT: u64 = (_IOC_WRITE | _IOC_READ) << _IOC_DIRSHIFT;

/// [`IOCSIZE_MASK`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const IOCSIZE_MASK: u64 = _IOC_SIZEMASK << _IOC_SIZESHIFT;

/// [`IOCSIZE_SHIFT`](https://github.com/torvalds/linux/blob/v5.6/include/uapi/asm-generic/ioctl.h)
pub const IOCSIZE_SHIFT: u64 = _IOC_SIZESHIFT;
