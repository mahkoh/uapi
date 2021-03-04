use testutils::*;
use uapi::*;

#[test]
fn accept4_() {
    let tmp = Tempdir::new();

    let server_path = format!("{}/server", tmp);

    let server_addr = super::addr(&server_path);

    let server = socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
    bind(*server, &server_addr).unwrap();
    listen(*server, 128).unwrap();

    let thread = std::thread::spawn(move || {
        for _ in 0..2 {
            let client = socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
            connect(*client, &server_addr).unwrap();
        }
    });

    let client = accept4(*server, sockaddr_none_mut(), 0).unwrap().0;
    assert_ne!(fcntl_getfd(*client).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let client = accept4(*server, sockaddr_none_mut(), c::SOCK_CLOEXEC)
        .unwrap()
        .0;
    assert_eq!(fcntl_getfd(*client).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    thread.join().unwrap();
}
