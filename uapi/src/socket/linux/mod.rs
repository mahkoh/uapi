use crate::*;

pub use netlink::*;

mod netlink;

#[man(accept4(2))]
pub fn accept4<T: Pod + ?Sized>(
    sockfd: c::c_int,
    mut addr: Option<&mut T>,
    flags: c::c_int,
) -> Result<(OwnedFd, usize)> {
    let mut addrlen = 0;
    let (ptr, len) = super::opt_to_sockaddr_mut(&mut addr, &mut addrlen)?;
    let res = unsafe { c::accept4(sockfd, ptr, len, flags) };
    let fd = map_err!(res).map(OwnedFd::new)?;
    Ok((fd, addrlen as usize))
}
