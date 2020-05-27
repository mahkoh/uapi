use std::{
    fs::File,
    io::{IoSlice, IoSliceMut, Read, Write},
    net::{TcpListener, TcpStream, UdpSocket},
    os::unix::net::{UnixDatagram, UnixListener, UnixStream},
    process::{Command, Stdio},
};
use uapi::*;

#[test]
fn fd() {
    assert_eq!(OwnedFd::new(-1).unwrap(), -1);
    assert_eq!(OwnedFd::new(-1).borrow(), Fd::new(-1));
    assert_eq!(*OwnedFd::new(-1), -1);
    assert_eq!(OwnedFd::new(-1).raw(), -1);

    assert_eq!(OwnedFd::from(File::from(OwnedFd::new(37))).unwrap(), 37);
    assert_eq!(
        OwnedFd::from(TcpListener::from(OwnedFd::new(37))).unwrap(),
        37
    );
    assert_eq!(
        OwnedFd::from(TcpStream::from(OwnedFd::new(37))).unwrap(),
        37
    );
    assert_eq!(
        OwnedFd::from(UdpSocket::from(OwnedFd::new(37))).unwrap(),
        37
    );
    assert_eq!(
        OwnedFd::from(UnixDatagram::from(OwnedFd::new(37))).unwrap(),
        37
    );
    assert_eq!(
        OwnedFd::from(UnixStream::from(OwnedFd::new(37))).unwrap(),
        37
    );
    assert_eq!(
        OwnedFd::from(UnixListener::from(OwnedFd::new(37))).unwrap(),
        37
    );

    let (mut r, w) = pipe().unwrap();

    Command::new("/bin/sh")
        .arg("-c")
        .arg("printf x")
        .stdout(w)
        .status()
        .unwrap();

    assert_eq!(&r.read_to_new_ustring().unwrap(), "x");

    let mut child = Command::new("/bin/sh")
        .arg("-c")
        .arg("printf y >&2; cat")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();

    OwnedFd::from(child.stdin.take().unwrap())
        .write_all(b"x")
        .unwrap();
    let mut r = OwnedFd::from(child.stdout.take().unwrap());
    assert_eq!(&r.read_to_new_ustring().unwrap(), "x");
    let mut r = OwnedFd::from(child.stderr.take().unwrap());
    assert_eq!(&r.read_to_new_ustring().unwrap(), "y");

    assert_eq!(Fd::new(-1).raw(), -1);
    assert_eq!(*Fd::new(-1), -1);

    assert_eq!(Fd::new(-1), Fd::new(-1));
    assert_eq!(OwnedFd::new(-1), OwnedFd::new(-1));
    assert_eq!(Fd::new(-1), OwnedFd::new(-1));
    assert_eq!(OwnedFd::new(-1), Fd::new(-1));

    let (mut r, mut w) = pipe().unwrap();
    assert_eq!(w.write(b"abc").unwrap(), 3);
    assert_eq!(w.borrow().write(b"abc").unwrap(), 3);
    assert_eq!(w.write_vectored(&[IoSlice::new(b"abc")]).unwrap(), 3);
    assert_eq!(
        w.borrow().write_vectored(&[IoSlice::new(b"abc")]).unwrap(),
        3
    );
    assert!(w.flush().is_ok());
    assert!(w.borrow().flush().is_ok());

    let mut buf = [0; 1];
    assert_eq!(r.read(&mut buf).unwrap(), 1);
    assert_eq!(buf[0], b'a');
    assert_eq!(r.borrow().read(&mut buf).unwrap(), 1);
    assert_eq!(buf[0], b'b');
    assert_eq!(
        r.read_vectored(&mut [IoSliceMut::new(&mut buf)]).unwrap(),
        1
    );
    assert_eq!(buf[0], b'c');
    assert_eq!(
        r.borrow()
            .read_vectored(&mut [IoSliceMut::new(&mut buf)])
            .unwrap(),
        1
    );
    assert_eq!(buf[0], b'a');
}
