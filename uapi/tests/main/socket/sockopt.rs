#[allow(unused_imports)]
use proc::*;
use std::mem;
use uapi::*;

#[test]
fn linger() {
    let socket = socket(c::AF_INET, c::SOCK_STREAM, 0).unwrap();

    let linger = c::linger {
        l_onoff: 0,
        l_linger: 0,
    };
    setsockopt(*socket, c::SOL_SOCKET, c::SO_LINGER, &linger).unwrap();

    let mut linger: c::linger = pod_zeroed();
    assert_eq!(
        getsockopt(*socket, c::SOL_SOCKET, c::SO_LINGER, &mut linger).unwrap(),
        mem::size_of::<c::linger>()
    );
    assert_eq!(linger.l_onoff, 0);

    let linger = c::linger {
        l_onoff: 1,
        l_linger: 0,
    };
    setsockopt(*socket, c::SOL_SOCKET, c::SO_LINGER, &linger).unwrap();

    let mut linger: c::linger = pod_zeroed();
    getsockopt(*socket, c::SOL_SOCKET, c::SO_LINGER, &mut linger).unwrap();
    assert_eq!(linger.l_onoff, 1);

    let linger = c::linger {
        l_onoff: 0,
        l_linger: 0,
    };
    setsockopt(*socket, c::SOL_SOCKET, c::SO_LINGER, &linger).unwrap();

    let mut linger: c::linger = pod_zeroed();
    getsockopt(*socket, c::SOL_SOCKET, c::SO_LINGER, &mut linger).unwrap();
    assert_eq!(linger.l_onoff, 0);
}
