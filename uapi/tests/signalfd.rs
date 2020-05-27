use uapi::*;

#[test]
#[cfg(target_os = "linux")]
fn signalfd_() {
    let sfd = signal_fd_new(&empty_sig_set().unwrap(), 0).unwrap();
    assert_ne!(fcntl_getfd(*sfd).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let sfd = signal_fd_new(&empty_sig_set().unwrap(), c::SFD_CLOEXEC | c::SFD_NONBLOCK)
        .unwrap();
    assert_eq!(fcntl_getfd(*sfd).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

    let mut set = empty_sig_set().unwrap();
    sigaddset(&mut set, c::SIGUSR1).unwrap();

    pthread_sigmask(c::SIG_BLOCK, Some(&set), None).unwrap();

    let mut buf = [pod_zeroed(); 5];
    assert_eq!(
        signal_fd_read(*sfd, &mut buf[..]).err().unwrap(),
        Errno(c::EAGAIN)
    );

    raise(c::SIGUSR1).unwrap();

    assert_eq!(
        signal_fd_read(*sfd, &mut buf[..]).err().unwrap(),
        Errno(c::EAGAIN)
    );

    signal_fd_mod(*sfd, &set).unwrap();

    let res = signal_fd_read(*sfd, &mut buf[..]).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].ssi_signo, c::SIGUSR1 as _);
}
