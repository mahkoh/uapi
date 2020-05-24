#![allow(non_snake_case)]

use crate::c::c_int;
use std::mem;

/// [`IOCPARAM_SHIFT`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOCPARM_SHIFT: u64 = 13;

/// [`IOCPARAM_SHIFT`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOCPARM_MASK: u64 = (1 << IOCPARM_SHIFT) - 1;

/// [`IOCPARM_LEN`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn IOCPARM_LEN(x: u64) -> u64 {
    (x >> 16) & IOCPARM_MASK
}
/// [`IOCBASECMD`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn IOCBASECMD(x: u64) -> u64 {
    x & !(IOCPARM_MASK << 16)
}
/// [`IOCGROUP`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn IOCGROUP(x: u64) -> u64 {
    (x >> 8) & 0xff
}

/// [`IOCPARM_MAX`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOCPARM_MAX: u64 = (1 << IOCPARM_SHIFT) - 1;

/// [`IOC_VOID`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOC_VOID: u64 = 0x20000000;

/// [`IOC_OUT`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOC_OUT: u64 = 0x40000000;

/// [`IOC_IN`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOC_IN: u64 = 0x80000000;

/// [`IOC_INOUT`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOC_INOUT: u64 = IOC_IN | IOC_OUT;

/// [`IOC_DIRMASK`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const IOC_DIRMASK: u64 = IOC_VOID | IOC_OUT | IOC_IN;

/// [`_IOC`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn _IOC(inout: u64, group: u64, num: u64, len: u64) -> u64 {
    inout | ((len & IOCPARM_MASK) << 16) | (group << 8) | (num)
}

/// [`_IO`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn _IO(g: u64, n: u64) -> u64 {
    _IOC(IOC_VOID, g, n, 0)
}

/// [`_IOWINT`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn _IOWINT(g: u64, n: u64) -> u64 {
    _IOC(IOC_VOID, g, n, mem::size_of::<c_int>() as _)
}

/// [`_IOR`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn _IOR<T>(g: u64, n: u64) -> u64 {
    _IOC(IOC_OUT, g, n, mem::size_of::<T>() as _)
}

/// [`_IOW`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn _IOW<T>(g: u64, n: u64) -> u64 {
    _IOC(IOC_IN, g, n, mem::size_of::<T>() as _)
}

/// [`_IOWR`][https://github.com/DragonFlyBSD/DragonFlyBSD/blob/v5.9.0/sys/sys/ioccom.h]
pub const fn _IOWR<T>(g: u64, n: u64) -> u64 {
    _IOC(IOC_INOUT, g, n, mem::size_of::<T>() as _)
}
