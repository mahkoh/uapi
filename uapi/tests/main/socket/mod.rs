use std::{
    collections::HashSet,
    io::{IoSlice, IoSliceMut, Read, Write},
    mem, thread,
};
use testutils::{strace, Tempdir};
use uapi::*;

mod cmsg;

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

    let mut server = {
        let fd = socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
        bind(*fd, &server_addr).unwrap();
        listen(*fd, 128).unwrap();
        fd
    };

    let thread = thread::spawn(move || {
        let mut client = socket(c::AF_UNIX, c::SOCK_STREAM, 0).unwrap();
        bind(*client, &client_addr).unwrap();
        connect(*client, &server_addr).unwrap();

        send(*client, b"hello world", 0).unwrap();
        shutdown(*client, c::SHUT_WR);

        assert_eq!(&client.read_to_new_ustring().unwrap(), "hol up");

        let mut pa: c::sockaddr_un = pod_zeroed();
        getpeername(*client, &mut pa).unwrap();
        assert_eq!(as_bytes(&pa), as_bytes(&server_addr));

        let mut sa: c::sockaddr_un = pod_zeroed();
        getsockname(*client, &mut sa).unwrap();
        assert_eq!(as_bytes(&sa), as_bytes(&client_addr));
    });

    let mut accepted_client_addr: c::sockaddr_un = pod_zeroed();
    let (mut client, addr_size) = strace(true, || {
        accept(*server, Some(&mut accepted_client_addr)).unwrap()
    });

    assert!(addr_size <= mem::size_of::<c::sockaddr_un>());
    assert_eq!(as_bytes(&accepted_client_addr), as_bytes(&client_addr));

    let mut buf = [0; 128];
    let len = recv(*client, &mut buf, 0).unwrap();
    assert_eq!(len, 11);
    assert_eq!(&buf[..11], b"hello world");
    client.write_all(b"hol up").unwrap();
    close(client);

    thread.join().unwrap();
}

#[test]
fn socket2() {
    let tmp = Tempdir::new();
    let server_path = &*format!("{}/server", tmp);
    let client_path = &*format!("{}/client", tmp);

    let server_addr = addr(server_path);
    let client_addr = addr(client_path);

    let mut server = {
        let fd = socket(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
        bind(*fd, &server_addr).unwrap();
        fd
    };

    {
        let mut client = socket(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
        bind(*client, &client_addr).unwrap();
        let msghdr = Msghdr {
            iov: &[IoSlice::new(b"hello world")],
            control: None,
            name: Some(&server_addr),
        };
        sendmsg(*client, &msghdr, 0).unwrap();
    }

    {
        let mut buf = [0; 128];
        let mut accepted_client_addr: c::sockaddr_un = pod_zeroed();
        let mut msghdr = MsghdrMut {
            iov: &mut [IoSliceMut::new(&mut buf)],
            control: None,
            name: Some(&mut accepted_client_addr),
            flags: 0,
        };
        let (buflen, addr_size) = recvmsg(*server, &mut msghdr, 0).unwrap();
        assert_eq!(buflen, 11);
        assert_eq!(&buf[..11], b"hello world");
        assert!(addr_size <= mem::size_of::<c::sockaddr_un>());
        assert_eq!(as_bytes(&accepted_client_addr), as_bytes(&client_addr));
    }

    unlink(client_path).unwrap();

    {
        let mut client = socket(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
        bind(*client, &client_addr).unwrap();
        sendto(*client, b"ayo", 0, &server_addr).unwrap();
    }

    {
        let mut buf = [0; 128];
        let mut accepted_client_addr: c::sockaddr_un = pod_zeroed();
        let (buflen, addr_size) =
            recvfrom(*server, &mut buf, 0, &mut accepted_client_addr).unwrap();
        assert_eq!(buflen, 3);
        assert_eq!(&buf[..3], b"ayo");
        assert!(addr_size <= mem::size_of::<c::sockaddr_un>());
        assert_eq!(as_bytes(&accepted_client_addr), as_bytes(&client_addr));
    }
}

#[test]
fn cmsg1() {
    let tmp = Tempdir::new();
    let f1 = open(format!("{}/a", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();
    let f2 = open(format!("{}/b", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();
    let f3 = open(format!("{}/c", tmp), c::O_CREAT | c::O_RDONLY, 0).unwrap();

    let mut inos: HashSet<c::ino_t> = [*f1, *f2, *f3]
        .iter()
        .map(|f| fstat(*f).unwrap())
        .map(|s| s.st_ino)
        .collect();

    let (a, b) = socketpair(c::AF_UNIX, c::SOCK_DGRAM, 0).unwrap();
    setsockopt_so_passcred(*b, 1).unwrap();

    {
        let mut buf = [0; 128];
        let len = {
            let mut buf = &mut buf[..];

            let mut hdr: c::cmsghdr = pod_zeroed();
            hdr.cmsg_level = c::SOL_SOCKET;
            hdr.cmsg_type = c::SCM_RIGHTS;

            let mut len = 0;
            len += cmsg_write(&mut buf, hdr, &[*f1, *f2]).unwrap();
            len += cmsg_write(&mut buf, hdr, &[*f3]).unwrap();
            len
        };

        let msghdr = Msghdr {
            iov: &[IoSlice::new(b"hello world")],
            control: Some(&buf[..len]),
            name: sockaddr_none_ref(),
        };
        sendmsg(*a, &msghdr, 0).unwrap();
    }

    {
        let mut data_buf = [0; 128];
        let mut cmsg_buf = [0; 128];
        let mut msghdr = MsghdrMut {
            iov: &mut [IoSliceMut::new(&mut data_buf)],
            control: Some(&mut cmsg_buf),
            name: sockaddr_none_mut(),
            flags: 0,
        };

        let (data_len, _) = recvmsg(*b, &mut msghdr, 0).unwrap();

        let mut cmsg: &[u8] = msghdr.control.take().unwrap();

        assert_eq!(data_len, 11);
        assert_eq!(&data_buf[..11], b"hello world");

        assert!(cmsg.len() > 0);

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

        assert!(saw_cred);
        assert!(inos.is_empty());
    }
}
