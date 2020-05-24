use std::panic::AssertUnwindSafe;
use uapi::*;

pub fn strace<T, F: FnOnce() -> T>(trace: bool, f: F) -> T {
    if !trace {
        return f();
    }

    let id = gettid();

    let start_efd = eventfd(0, 0).unwrap();
    let start_efd_copy = start_efd.borrow();
    let stop_efd = eventfd(0, 0).unwrap();
    let stop_efd_copy = stop_efd.borrow();

    std::thread::spawn(move || {
        let mut command = std::process::Command::new("strace")
            // .arg("-f")
            .arg("-p")
            .arg(id.to_string())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        let mut notified_parent = false;
        let stderr = OwnedFd::from(command.stderr.take().unwrap());
        fcntl_setfl(*stderr, fcntl_getfl(*stderr).unwrap() | c::O_NONBLOCK).unwrap();
        let mut buf = [0; 1024];
        let mut pipe = || loop {
            let res = read(*stderr, &mut buf);
            match res {
                Ok(n) => {
                    std::io::Write::write_all(&mut Fd::new(1), &buf[..n]).unwrap();
                    if !notified_parent {
                        notified_parent = true;
                        eventfd_write(*start_efd_copy, 1).unwrap();
                    }
                }
                Err(Errno(c::EAGAIN)) => break,
                e => {
                    e.unwrap();
                }
            };
        };
        loop {
            let mut fds = [
                c::pollfd {
                    fd: *stderr,
                    events: c::POLLIN | c::POLLHUP,
                    revents: 0,
                },
                c::pollfd {
                    fd: *stop_efd_copy,
                    events: c::POLLIN | c::POLLHUP,
                    revents: 0,
                },
            ];
            poll(&mut fds, 0).unwrap();
            if fds[0].revents & c::POLLHUP != 0 || fds[1].revents != 0 {
                break;
            }
            if fds[0].revents & c::POLLIN != 0 {
                pipe();
            }
        }
        command.kill().unwrap();
        pipe();
        eventfd_write(*start_efd_copy, 1).unwrap();
    });

    eventfd_read(*start_efd).unwrap();

    let res = std::panic::catch_unwind(AssertUnwindSafe(f));

    eventfd_write(*stop_efd, 1).unwrap();
    eventfd_read(*start_efd).unwrap();

    match res {
        Ok(r) => r,
        Err(p) => std::panic::resume_unwind(p),
    }
}
