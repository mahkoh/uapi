#[allow(unused_imports)]
use proc::*;
use uapi::*;

fn switch(
    socket: c::c_int,
    switch: fn(c::c_int, c::c_int) -> Result<()>,
    get: fn(c::c_int) -> Result<c::c_int>,
) {
    switch(socket, 0).unwrap();
    assert_eq!(get(socket).unwrap(), 0);
    switch(socket, 1).unwrap();
    assert_ne!(get(socket).unwrap(), 0);
    switch(socket, 0).unwrap();
    assert_eq!(get(socket).unwrap(), 0);
}

fn switch8(
    socket: c::c_int,
    switch: fn(c::c_int, u8) -> Result<()>,
    get: fn(c::c_int) -> Result<u8>,
) {
    switch(socket, 0).unwrap();
    assert_eq!(get(socket).unwrap(), 0);
    switch(socket, 1).unwrap();
    assert_ne!(get(socket).unwrap(), 0);
    switch(socket, 0).unwrap();
    assert_eq!(get(socket).unwrap(), 0);
}

#[test]
fn on_off() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    switch(*socket, setsockopt_so_reuseaddr, getsockopt_so_reuseaddr);
    switch(*socket, setsockopt_so_reuseport, getsockopt_so_reuseport);
    switch(*socket, setsockopt_tcp_nodelay, getsockopt_tcp_nodelay);
    switch(*socket, setsockopt_so_oobinline, getsockopt_so_oobinline);
    switch(*socket, setsockopt_so_keepalive, getsockopt_so_keepalive);
    switch(*socket, setsockopt_so_timestamp, getsockopt_so_timestamp);
    #[cfg(any(target_os = "android", target_os = "linux"))]
    switch(*socket, setsockopt_so_passcred, getsockopt_so_passcred);
    #[cfg(not(any(target_os = "freebsd", target_os = "openbsd")))]
    switch(*socket, setsockopt_ip_pktinfo, getsockopt_ip_pktinfo);
}

#[test]
fn on_off2() {
    let socket = socket(c::AF_INET, c::SOCK_DGRAM, 0).unwrap();

    switch(*socket, setsockopt_so_broadcast, getsockopt_so_broadcast);
    switch8(
        *socket,
        setsockopt_ip_multicast_loop,
        getsockopt_ip_multicast_loop,
    );
}

#[test]
fn on_off6() {
    let socket = socket(c::AF_INET6, c::SOCK_STREAM, 0).unwrap();

    switch(
        *socket,
        setsockopt_ipv6_recvpktinfo,
        getsockopt_ipv6_recvpktinfo,
    );
}

#[test_if(root)]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn on_off_root() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    switch(
        *socket,
        setsockopt_ip_transparent,
        getsockopt_ip_transparent,
    );
    switch(*socket, setsockopt_so_mark, getsockopt_so_mark);
}

#[test]
fn linger() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    setsockopt_so_linger(
        *socket,
        c::linger {
            l_onoff: 0,
            l_linger: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_linger(*socket).unwrap().l_onoff, 0);
    setsockopt_so_linger(
        *socket,
        c::linger {
            l_onoff: 1,
            l_linger: 0,
        },
    )
    .unwrap();
    assert_ne!(getsockopt_so_linger(*socket).unwrap().l_onoff, 0);
    setsockopt_so_linger(
        *socket,
        c::linger {
            l_onoff: 0,
            l_linger: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_linger(*socket).unwrap().l_onoff, 0);
}

#[test]
#[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
fn acceptconn() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    assert_eq!(getsockopt_so_acceptconn(*socket).unwrap(), 0);
    listen(*socket, 128).unwrap();
    assert_ne!(getsockopt_so_acceptconn(*socket).unwrap(), 0);
}

#[test]
#[cfg(not(any(target_os = "macos", target_os = "freebsd", target_os = "openbsd")))]
fn membership() {
    use std::mem;

    let socket = socket(c::AF_INET, c::SOCK_DGRAM, 0).unwrap();

    let mut reqn: c::ip_mreqn = unsafe { mem::zeroed() };
    reqn.imr_multiaddr.s_addr = 224u32.to_le();

    setsockopt_ip_add_membership(*socket, reqn).unwrap();
    setsockopt_ip_drop_membership(*socket, reqn).unwrap();
}

#[test]
#[cfg(target_os = "linux")]
fn rcvbuf() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    let buf = getsockopt_so_rcvbuf(*socket).unwrap();
    setsockopt_so_rcvbuf(*socket, buf / 2 + 1).unwrap();
    assert_eq!(getsockopt_so_rcvbuf(*socket).unwrap(), buf + 2);
    setsockopt_so_rcvbuf(*socket, buf / 2).unwrap();
    assert_eq!(getsockopt_so_rcvbuf(*socket).unwrap(), buf);

    let buf = getsockopt_so_sndbuf(*socket).unwrap();
    setsockopt_so_sndbuf(*socket, buf / 2 + 1).unwrap();
    assert_eq!(getsockopt_so_sndbuf(*socket).unwrap(), buf + 2);
    setsockopt_so_sndbuf(*socket, buf / 2).unwrap();
    assert_eq!(getsockopt_so_sndbuf(*socket).unwrap(), buf);
}

#[test]
#[cfg(not(target_os = "linux"))]
fn rcvbuf() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    let buf = getsockopt_so_rcvbuf(*socket).unwrap();
    setsockopt_so_rcvbuf(*socket, buf + 1).unwrap();
    assert_eq!(getsockopt_so_rcvbuf(*socket).unwrap(), buf + 1);
    setsockopt_so_rcvbuf(*socket, buf).unwrap();
    assert_eq!(getsockopt_so_rcvbuf(*socket).unwrap(), buf);

    let buf = getsockopt_so_sndbuf(*socket).unwrap();
    setsockopt_so_sndbuf(*socket, buf + 1).unwrap();
    assert_eq!(getsockopt_so_sndbuf(*socket).unwrap(), buf + 1);
    setsockopt_so_sndbuf(*socket, buf).unwrap();
    assert_eq!(getsockopt_so_sndbuf(*socket).unwrap(), buf);
}

#[test_if(root)]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn rcvbuf_force() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    let buf = getsockopt_so_rcvbuf(*socket).unwrap();
    setsockopt_so_rcvbufforce(*socket, buf / 2 + 1).unwrap();
    assert_eq!(getsockopt_so_rcvbuf(*socket).unwrap(), buf + 2);
    setsockopt_so_rcvbufforce(*socket, buf / 2).unwrap();
    assert_eq!(getsockopt_so_rcvbuf(*socket).unwrap(), buf);

    let buf = getsockopt_so_sndbuf(*socket).unwrap();
    setsockopt_so_sndbufforce(*socket, buf / 2 + 1).unwrap();
    assert_eq!(getsockopt_so_sndbuf(*socket).unwrap(), buf + 2);
    setsockopt_so_sndbufforce(*socket, buf / 2).unwrap();
    assert_eq!(getsockopt_so_sndbuf(*socket).unwrap(), buf);
}

#[cfg(any(
    target_os = "android",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "nacl"
))]
#[test]
fn tcp_keepidle() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    setsockopt_tcp_keepidle(*socket, 1).unwrap();
    assert_eq!(getsockopt_tcp_keepidle(*socket).unwrap(), 1);
    setsockopt_tcp_keepidle(*socket, 2).unwrap();
    assert_eq!(getsockopt_tcp_keepidle(*socket).unwrap(), 2);
    setsockopt_tcp_keepidle(*socket, 3).unwrap();
    assert_eq!(getsockopt_tcp_keepidle(*socket).unwrap(), 3);
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn original_dst() {
    let server = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();
    let mut addr: c::sockaddr_in = pod_zeroed();
    bind(*server, &addr).unwrap();
    getsockname(*server, &mut addr).unwrap();
    listen(*server, 128).unwrap();

    std::thread::spawn(move || {
        let client = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();
        connect(*client, &addr).unwrap();
    });

    let client = accept(*server, sockaddr_none_mut()).unwrap().0;

    getsockopt_so_original_dst(*client).unwrap();
}

#[test]
fn type_() {
    let server = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    assert_eq!(getsockopt_so_type(*server).unwrap(), c::SOCK_STREAM);
}

#[test]
fn multicast_ttl() {
    let socket = socket(c::AF_INET, c::SOCK_DGRAM, 0).unwrap();

    setsockopt_ip_multicast_ttl(*socket, 1).unwrap();
    assert_eq!(getsockopt_ip_multicast_ttl(*socket).unwrap(), 1);
    setsockopt_ip_multicast_ttl(*socket, 2).unwrap();
    assert_eq!(getsockopt_ip_multicast_ttl(*socket).unwrap(), 2);
    setsockopt_ip_multicast_ttl(*socket, 3).unwrap();
    assert_eq!(getsockopt_ip_multicast_ttl(*socket).unwrap(), 3);
}

#[test]
fn timeo() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    setsockopt_so_rcvtimeo(
        *socket,
        c::timeval {
            tv_sec: 1,
            tv_usec: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_rcvtimeo(*socket).unwrap().tv_sec, 1);
    setsockopt_so_rcvtimeo(
        *socket,
        c::timeval {
            tv_sec: 2,
            tv_usec: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_rcvtimeo(*socket).unwrap().tv_sec, 2);
    setsockopt_so_rcvtimeo(
        *socket,
        c::timeval {
            tv_sec: 3,
            tv_usec: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_rcvtimeo(*socket).unwrap().tv_sec, 3);

    setsockopt_so_sndtimeo(
        *socket,
        c::timeval {
            tv_sec: 1,
            tv_usec: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_sndtimeo(*socket).unwrap().tv_sec, 1);
    setsockopt_so_sndtimeo(
        *socket,
        c::timeval {
            tv_sec: 2,
            tv_usec: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_sndtimeo(*socket).unwrap().tv_sec, 2);
    setsockopt_so_sndtimeo(
        *socket,
        c::timeval {
            tv_sec: 3,
            tv_usec: 0,
        },
    )
    .unwrap();
    assert_eq!(getsockopt_so_sndtimeo(*socket).unwrap().tv_sec, 3);
}

#[test]
fn error() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    assert_eq!(getsockopt_so_error(*socket).unwrap(), 0);
}

#[test]
#[cfg(any(target_os = "android", target_os = "linux"))]
fn membership6() {
    use std::mem;

    let socket = socket(c::AF_INET6, c::SOCK_DGRAM, 0).unwrap();

    let mut reqn: c::ipv6_mreq = unsafe { mem::zeroed() };
    reqn.ipv6mr_multiaddr.s6_addr[0] = 0xff;

    setsockopt_ipv6_add_membership(*socket, reqn).unwrap();
    setsockopt_ipv6_drop_membership(*socket, reqn).unwrap();
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[test]
fn peercred() {
    let (a, _) = socketpair(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();

    let cred: c::ucred = getsockopt_so_peercred(*a).unwrap();
    assert_eq!(cred.pid, getpid());
    assert_eq!(cred.uid, geteuid());
    assert_eq!(cred.gid, getegid());
}
