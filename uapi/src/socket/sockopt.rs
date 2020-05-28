pub use fuckrustfmt::*;

/// fuck rustfmt for forcing me to reorganize my code just to disable it on a section
#[rustfmt::skip]
mod fuckrustfmt {
    use crate::*;
    use std::mem;

    /*
    sock_opt produces setsockopt_* and getsockopt_* functions

    the arguments are as follows:

        1. attributes. these attributes are applied immediately after the man-page reference
        2. one of get/set/bi. depending on which functions you want to generate
        3. the level
        4. the optname
        5. named arguments:
            - ty: specifies the type of the option value. default is int

    note that the attributes are not followed by a comma
     */

    sock_opt!(get, SOL_SOCKET, SO_ACCEPTCONN);
    sock_opt!(bi, SOL_SOCKET, SO_LINGER, ty = c::linger);
    sock_opt!(bi, SOL_SOCKET, SO_REUSEADDR);
    sock_opt!(bi, SOL_SOCKET, SO_REUSEPORT);
    sock_opt!(bi, IPPROTO_TCP, TCP_NODELAY);
    sock_opt!(bi, IPPROTO_IP, IP_MULTICAST_LOOP);
    sock_opt!(bi, SOL_SOCKET, SO_BROADCAST);
    sock_opt!(bi, SOL_SOCKET, SO_OOBINLINE);
    sock_opt!(bi, SOL_SOCKET, SO_KEEPALIVE);
    sock_opt!(bi, SOL_SOCKET, SO_TIMESTAMP);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(bi, SOL_IP, IP_TRANSPARENT);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(bi, SOL_SOCKET, SO_PASSCRED);
    #[cfg(any(target_os = "android", target_os = "ios", target_os = "linux", target_os = "macos", target_os = "netbsd"))]
    sock_opt!(bi, IPPROTO_IP, IP_PKTINFO);
    #[cfg(any(target_os = "android", target_os = "freebsd", target_os = "ios", target_os = "linux", target_os = "macos", target_os = "netbsd", target_os = "openbsd"))]
    sock_opt!(bi, IPPROTO_IPV6, IPV6_RECVPKTINFO);
    #[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
    sock_opt!(set, IPPROTO_IP, IP_ADD_MEMBERSHIP, ty = c::ip_mreqn);
    #[cfg(not(any(target_os = "macos", target_os = "freebsd")))]
    sock_opt!(set, IPPROTO_IP, IP_DROP_MEMBERSHIP, ty = c::ip_mreqn);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(get, SOL_IP, SO_ORIGINAL_DST, ty = c::sockaddr_in);
    sock_opt!(bi, SOL_SOCKET, SO_RCVBUF);
    sock_opt!(bi, SOL_SOCKET, SO_SNDBUF);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(set, SOL_SOCKET, SO_RCVBUFFORCE);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(set, SOL_SOCKET, SO_SNDBUFFORCE);
    #[cfg(any(target_os = "android", target_os = "dragonfly", target_os = "freebsd", target_os = "linux", target_os = "nacl"))]
    sock_opt!(bi, IPPROTO_TCP, TCP_KEEPIDLE);
    #[cfg(target_os = "linux")]
    sock_opt!(bi, SOL_SOCKET, SO_MARK);
    sock_opt!(get, SOL_SOCKET, SO_TYPE);
    sock_opt!(bi, IPPROTO_IP, IP_MULTICAST_TTL);
    sock_opt!(bi, SOL_SOCKET, SO_RCVTIMEO, ty = c::timeval);
    sock_opt!(bi, SOL_SOCKET, SO_SNDTIMEO, ty = c::timeval);
    sock_opt!(get, SOL_SOCKET, SO_ERROR);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(set, IPPROTO_IPV6, IPV6_ADD_MEMBERSHIP, ty = c::ipv6_mreq);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(set, IPPROTO_IPV6, IPV6_DROP_MEMBERSHIP, ty = c::ipv6_mreq);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    sock_opt!(get, SOL_SOCKET, SO_PEERCRED, ty = c::ucred);
}
