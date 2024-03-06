use std::{
    collections::HashSet,
    io::{IoSlice, IoSliceMut, Write},
    mem,
    mem::MaybeUninit,
    thread,
};
use testutils::*;
use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

mod cmsg;
mod sockopt;

fn addr(s: &str) -> c::sockaddr_un {
    let mut addr: c::sockaddr_un = pod_zeroed();
    addr.sun_family = c::AF_UNIX as _;
    pod_write(s.as_bytes(), &mut addr.sun_path[..s.len()]).unwrap();
    addr
}

#[test]
fn socket1() {
    let tmp = Tempdir::new();
    let server_path = &*format!("{}/server", tmp);
    let client_path = &*format!("{}/client", tmp);

    let server_addr = addr(server_path);
    let client_addr = addr(client_path);

    let server = {
        let fd = socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
        bind(*fd, &server_addr).unwrap();
        listen(*fd, 128).unwrap();
        fd
    };

    let thread = thread::spawn(move || {
        let mut client = socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
        bind(*client, &client_addr).unwrap();
        connect(*client, &server_addr).unwrap();

        let mut pa: c::sockaddr_un = pod_zeroed();
        getpeername(*client, &mut pa).unwrap();
        cmp_addr_un(&pa, &server_addr);

        let mut sa: c::sockaddr_un = pod_zeroed();
        getsockname(*client, &mut sa).unwrap();
        cmp_addr_un(&sa, &client_addr);

        send(*client, b"hello world", 0).unwrap();
        shutdown(*client, c::SHUT_WR).unwrap();

        assert_eq!(&client.read_to_new_ustring().unwrap(), "hol up");
    });

    let mut accepted_client_addr: c::sockaddr_un = pod_zeroed();
    let (mut client, addr_size) =
        accept(*server, Some(&mut accepted_client_addr)).unwrap();

    assert!(addr_size <= mem::size_of::<c::sockaddr_un>());
    cmp_addr_un(&accepted_client_addr, &client_addr);

    let mut buf = [0u8; 128];
    let buf = recv(*client, &mut buf[..], 0).unwrap();
    assert_eq!(buf, b"hello world");
    client.write_all(b"hol up").unwrap();
    shutdown(*client, c::SHUT_WR).unwrap();

    thread.join().unwrap();
}

fn cmp_addr_un(a: &c::sockaddr_un, b: &c::sockaddr_un) {
    assert_eq!(a.sun_family, b.sun_family);
    assert_eq!(&a.sun_path[..], &b.sun_path[..]);
}

#[test]
fn socket2() {
    let tmp = Tempdir::new();
    let server_path = &*format!("{}/server", tmp);
    let client_path = &*format!("{}/client", tmp);

    let server_addr = addr(server_path);
    let client_addr = addr(client_path);

    let server = {
        let fd = socket(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
        bind(*fd, &server_addr).unwrap();
        fd
    };

    {
        let client = socket(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
        bind(*client, &client_addr).unwrap();
        let msghdr = Msghdr {
            iov: &[IoSlice::new(b"hello world")][..],
            control: msghdr_control_none_ref(),
            name: Some(&server_addr),
        };
        sendmsg(*client, &msghdr, 0).unwrap();
    }

    {
        let mut buf = [0; 128];
        let mut accepted_client_addr: c::sockaddr_un = pod_zeroed();
        let mut msghdr = MsghdrMut {
            iov: &mut [IoSliceMut::new(&mut buf)][..],
            control: msghdr_control_none_mut(),
            name: Some(&mut accepted_client_addr),
            flags: 0,
        };
        let (buf, addr_size, _) = recvmsg(*server, &mut msghdr, 0).unwrap();
        let mut buf = buf.iter();
        assert_eq!(buf.next(), Some("hello world".as_bytes()));
        assert_eq!(buf.next(), None);
        assert!(addr_size <= mem::size_of::<c::sockaddr_un>());
        cmp_addr_un(&accepted_client_addr, &client_addr);
    }

    unlink(client_path).unwrap();

    {
        let client = socket(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
        bind(*client, &client_addr).unwrap();
        sendto(*client, b"ayo", 0, &server_addr).unwrap();
    }

    {
        let mut buf = [MaybeUninit::<u8>::uninit(); 128];
        let mut accepted_client_addr: c::sockaddr_un = pod_zeroed();
        let (buf, addr_size) =
            recvfrom(*server, &mut buf[..], 0, &mut accepted_client_addr).unwrap();
        assert_eq!(buf, b"ayo");
        assert!(addr_size <= mem::size_of::<c::sockaddr_un>());
        cmp_addr_un(&accepted_client_addr, &client_addr);
    }
}

#[test]
fn cmsg1() {
    let tmp = Tempdir::new();
    let f1 = open(format!("{}/a", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();
    let f2 = open(format!("{}/b", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();
    #[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
    let f3 = open(format!("{}/c", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();

    let mut inos: HashSet<c::ino_t> = [
        *f1,
        *f2,
        #[cfg(not(any(target_os = "macos", target_os = "openbsd")))]
        *f3,
    ]
    .iter()
    .map(|f| fstat(*f).unwrap())
    .map(|s| s.st_ino)
    .collect();

    let (a, b) = socketpair(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();

    #[cfg(any(target_os = "android", target_os = "linux"))]
    setsockopt(*b, c::SOL_SOCKET, c::SO_PASSCRED, &1i32).unwrap();

    {
        let mut buf = [MaybeUninit::uninit(); 128];
        let len = {
            let mut buf = &mut buf[..];

            let mut hdr: c::cmsghdr = pod_zeroed();
            hdr.cmsg_level = c::SOL_SOCKET;
            hdr.cmsg_type = c::SCM_RIGHTS;

            let mut len = 0;
            len += cmsg_write(&mut buf, hdr, &[*f1, *f2]).unwrap();
            cfg_if::cfg_if! {
                if #[cfg(not(any(target_os = "macos", target_os = "openbsd")))] {
                    len += cmsg_write(&mut buf, hdr, &[*f3]).unwrap();
                }
            }
            len
        };

        let msghdr = Msghdr {
            iov: &[IoSlice::new(b"hello world")][..],
            control: Some(&buf[..len]),
            name: sockaddr_none_ref(),
        };
        sendmsg(*a, &msghdr, 0).unwrap();
    }

    {
        let mut data_buf = [0; 128];
        let mut cmsg_buf = [0; 128];
        let mut msghdr = MsghdrMut {
            iov: &mut [IoSliceMut::new(&mut data_buf)][..],
            control: Some(&mut cmsg_buf[..]),
            name: sockaddr_none_mut(),
            flags: 0,
        };

        let (data, _, mut cmsg) = recvmsg(*b, &mut msghdr, 0).unwrap();
        let mut data = data.iter();

        assert_eq!(data.next(), Some("hello world".as_bytes()));
        assert_eq!(data.next(), None);

        assert!(cmsg.len() > 0);

        #[cfg(any(target_os = "android", target_os = "linux"))]
        let mut saw_cred = false;

        while cmsg.len() > 0 {
            let (_, hdr, data) = cmsg_read(&mut cmsg).unwrap();

            match (hdr.cmsg_level, hdr.cmsg_type) {
                (c::SOL_SOCKET, c::SCM_RIGHTS) => {
                    let data: Vec<_> = pod_iter::<c::c_int, _>(data)
                        .unwrap()
                        .map(OwnedFd::new)
                        .collect();
                    for fd in data {
                        assert!(inos.remove(&fstat(*fd).unwrap().st_ino));
                    }
                }
                #[cfg(any(target_os = "android", target_os = "linux"))]
                (c::SOL_SOCKET, c::SCM_CREDENTIALS) => {
                    let data: c::ucred = pod_read(data).unwrap();
                    assert_eq!(data.pid, getpid());
                    assert_eq!(data.uid, geteuid());
                    assert_eq!(data.gid, getegid());
                    saw_cred = true;
                }
                _ => panic!("unexpected cmsg_level"),
            }
        }

        #[cfg(any(target_os = "android", target_os = "linux"))]
        assert!(saw_cred);

        assert!(inos.is_empty());
    }
}
