use crate::*;
use cfg_if::cfg_if;
use std::{
    convert::TryInto,
    io::{IoSlice, IoSliceMut},
    mem, ptr,
};

pub use cmsg::*;
pub use sockopt::*;

mod cmsg;
mod sockopt;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

/// Marker trait for socket address types
///
/// # Safety
///
/// If a type `T` implements this trait then it must be one of the socket types supported
/// by the operating system.
pub unsafe trait SockAddr: Pod {}

macro_rules! imp {
    ($($path:path)*) => {
        $(unsafe impl SockAddr for $path {})*
    }
}

imp! {
    c::sockaddr
    c::sockaddr_storage
    c::sockaddr_un
    c::sockaddr_in
    c::sockaddr_in6
    c::sockaddr_nl
    c::sockaddr_alg
    c::sockaddr_ll
    c::sockaddr_vm
}

#[man(socket(2))]
pub fn socket(domain: c::c_int, ty: c::c_int, protocol: c::c_int) -> Result<OwnedFd> {
    let res = unsafe { c::socket(domain, ty, protocol) };
    map_err!(res).map(OwnedFd::new)
}

#[man(socketpair(2))]
pub fn socketpair(
    domain: c::c_int,
    ty: c::c_int,
    protocol: c::c_int,
) -> Result<(OwnedFd, OwnedFd)> {
    let mut socks = [0, 0];
    let res = unsafe { c::socketpair(domain, ty, protocol, &mut socks[0]) };
    map_err!(res).map(|_| (OwnedFd::new(socks[0]), OwnedFd::new(socks[1])))
}

#[man(bind(2))]
pub fn bind<T: SockAddr + ?Sized>(sockfd: c::c_int, addr: &T) -> Result<()> {
    let res = unsafe { c::bind(sockfd, addr as *const _ as *const _, to_addrlen(addr)?) };
    map_err!(res).map(drop)
}

#[man(connect(2))]
pub fn connect<T: SockAddr + ?Sized>(sockfd: c::c_int, addr: &T) -> Result<()> {
    let res =
        unsafe { c::connect(sockfd, addr as *const _ as *const _, to_addrlen(addr)?) };
    map_err!(res).map(drop)
}

#[man(listen(2))]
pub fn listen(sockfd: c::c_int, backlog: c::c_int) -> Result<()> {
    let res = unsafe { c::listen(sockfd, backlog) };
    map_err!(res).map(drop)
}

fn to_addrlen<T: ?Sized>(t: &T) -> Result<c::socklen_t> {
    mem::size_of_val(t).try_into().map_err(|_| Errno(c::EINVAL))
}

fn opt_to_sockaddr<T: ?Sized>(
    addr: Option<&T>,
) -> Result<(*mut c::sockaddr, c::socklen_t)> {
    match addr {
        Some(addr) => Ok((addr as *const _ as *mut _, to_addrlen(addr)?)),
        _ => Ok((ptr::null_mut(), 0)),
    }
}

fn opt_to_sockaddr_mut<T: ?Sized>(
    addr: &mut Option<&mut T>,
    addrlen: &mut c::socklen_t,
) -> Result<(*mut c::sockaddr, *mut c::socklen_t)> {
    match *addr {
        Some(ref mut addr) => {
            *addrlen = to_addrlen(*addr)?;
            Ok((*addr as *mut _ as *mut _, addrlen as *mut _))
        }
        _ => Ok((ptr::null_mut(), ptr::null_mut())),
    }
}

#[man(accept(2))]
pub fn accept<T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    mut addr: Option<&mut T>,
) -> Result<(OwnedFd, usize)> {
    let mut addrlen = 0;
    let (ptr, len) = opt_to_sockaddr_mut(&mut addr, &mut addrlen)?;
    let res = unsafe { c::accept(sockfd, ptr, len) };
    let fd = map_err!(res).map(OwnedFd::new)?;
    black_box(ptr);
    Ok((fd, addrlen as usize))
}

#[man(getsockname(2))]
pub fn getsockname<T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    addr: &mut T,
) -> Result<c::socklen_t> {
    let mut addrlen = to_addrlen(addr)?;
    let res = unsafe { c::getsockname(sockfd, addr as *mut _ as *mut _, &mut addrlen) };
    black_box(addr);
    map_err!(res).map(|_| addrlen)
}

#[man(getpeername(2))]
pub fn getpeername<T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    addr: &mut T,
) -> Result<usize> {
    let mut addrlen = to_addrlen(addr)?;
    let res = unsafe { c::getpeername(sockfd, addr as *mut _ as *mut _, &mut addrlen) };
    black_box(addr);
    map_err!(res).map(|_| addrlen as usize)
}

#[man(recv(2))]
pub fn recv(sockfd: c::c_int, buf: &mut [u8], flags: c::c_int) -> Result<usize> {
    let res = unsafe { c::recv(sockfd, buf as *mut _ as *mut _, buf.len(), flags) };
    map_err!(res).map(|v| v as usize)
}

#[man(recvfrom(2))]
pub fn recvfrom<T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    buf: &mut [u8],
    flags: c::c_int,
    addr: &mut T,
) -> Result<(usize, usize)> {
    let mut addrlen = to_addrlen(addr)?;
    let res = unsafe {
        c::recvfrom(
            sockfd,
            buf as *mut _ as *mut _,
            buf.len(),
            flags,
            addr as *mut _ as *mut _,
            &mut addrlen,
        )
    };
    black_box(addr);
    map_err!(res).map(|v| (v as usize, addrlen as _))
}

#[man(send(2))]
pub fn send(sockfd: c::c_int, buf: &[u8], flags: c::c_int) -> Result<usize> {
    let res = unsafe { c::send(sockfd, buf as *const _ as *const _, buf.len(), flags) };
    map_err!(res).map(|v| v as usize)
}

#[man(sendto(2))]
pub fn sendto<T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    buf: &[u8],
    flags: c::c_int,
    addr: &T,
) -> Result<usize> {
    let res = unsafe {
        c::sendto(
            sockfd,
            buf as *const _ as *const _,
            buf.len(),
            flags,
            addr as *const _ as *const _,
            to_addrlen(addr)?,
        )
    };
    map_err!(res).map(|v| v as usize)
}

#[man(shutdown(2))]
pub fn shutdown(sockfd: c::c_int, how: c::c_int) -> Result<()> {
    let res = unsafe { c::shutdown(sockfd, how) };
    map_err!(res).map(drop)
}

/// Returns `Option::<&c::sockaddr>::None`
///
/// This is useful for functions or structures which are generic over the sockaddr type
/// and whose type cannot be inferred if `None` is used on its own.
pub fn sockaddr_none_ref<'a>() -> Option<&'a c::sockaddr> {
    None
}

/// Returns `Option::<&mut c::sockaddr>::None`
///
/// This is useful for functions or structures which are generic over the sockaddr type
/// and whose type cannot be inferred if `None` is used on its own.
pub fn sockaddr_none_mut<'a>() -> Option<&'a mut c::sockaddr> {
    None
}

/// Rusty version of a mutable `c::sockaddr`
///
/// Use `sockaddr_none_mut` to avoid type inference errors
pub struct MsghdrMut<'a, 'b, 'c, 'd, T: SockAddr + ?Sized = c::sockaddr> {
    pub iov: &'b mut [IoSliceMut<'a>],
    pub control: Option<&'c mut [u8]>,
    pub name: Option<&'d mut T>,
    pub flags: c::c_int,
}

#[man(recvmsg(2))]
pub fn recvmsg<T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    msghdr: &mut MsghdrMut<T>,
    flags: c::c_int,
) -> Result<(usize, usize)> {
    let mut sockaddr_len = 0;
    let (sockaddr_ptr, _) = opt_to_sockaddr_mut(&mut msghdr.name, &mut sockaddr_len)?;

    let mut c_msghdr: c::msghdr = pod_zeroed();
    c_msghdr.msg_iov = msghdr.iov.as_mut_ptr() as *mut _;
    c_msghdr.msg_iovlen = msghdr.iov.len();
    c_msghdr.msg_control = msghdr
        .control
        .as_mut()
        .map(|b| b.as_mut_ptr() as *mut _)
        .unwrap_or(ptr::null_mut());
    c_msghdr.msg_controllen = msghdr.control.as_ref().map(|b| b.len()).unwrap_or(0);
    c_msghdr.msg_name = sockaddr_ptr as *mut _;
    c_msghdr.msg_namelen = sockaddr_len;

    let res = unsafe { c::recvmsg(sockfd, &mut c_msghdr, flags) };
    map_err!(res)?;

    if let Some(control) = msghdr.control.as_mut() {
        *control = &mut mem::replace(control, &mut [])[..c_msghdr.msg_controllen];
    }
    msghdr.flags = c_msghdr.msg_flags;

    Ok((res as usize, c_msghdr.msg_namelen as usize))
}

/// Rusty version of an immutable `c::sockaddr`
///
/// Use `sockaddr_none_ref` to avoid type inference errors
pub struct Msghdr<'a, T: SockAddr + ?Sized> {
    pub iov: &'a [IoSlice<'a>],
    pub control: Option<&'a [u8]>,
    pub name: Option<&'a T>,
}

#[man(sendmsg(2))]
pub fn sendmsg<'a, T: SockAddr + ?Sized>(
    sockfd: c::c_int,
    msghdr: &'a Msghdr<'a, T>,
    flags: c::c_int,
) -> Result<usize> {
    let (sockaddr_ptr, sockaddr_len) = opt_to_sockaddr(msghdr.name)?;

    let mut c_msghdr: c::msghdr = pod_zeroed();
    c_msghdr.msg_iov = msghdr.iov.as_ptr() as *mut _;
    c_msghdr.msg_iovlen = msghdr.iov.len();
    c_msghdr.msg_control = msghdr
        .control
        .map(|b| b.as_ptr() as *mut _)
        .unwrap_or(ptr::null_mut());
    c_msghdr.msg_controllen = msghdr.control.map(|b| b.len()).unwrap_or(0);
    c_msghdr.msg_name = sockaddr_ptr as *mut _;
    c_msghdr.msg_namelen = sockaddr_len;

    let res = unsafe { c::sendmsg(sockfd, &c_msghdr, flags) };
    map_err!(res)?;

    Ok(res as usize)
}
