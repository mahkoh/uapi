use uapi::*;

#[test]
fn signal_() {
    let mut set = empty_sig_set().unwrap();

    pthread_sigmask(c::SIG_SETMASK, Some(&set), None).unwrap();

    sigaddset(&mut set, c::SIGUSR1).unwrap();
    let mut oldset = empty_sig_set().unwrap();

    pthread_sigmask(c::SIG_SETMASK, Some(&set), Some(&mut oldset)).unwrap();

    assert!(!sigismember(&oldset, c::SIGUSR1).unwrap());

    sigemptyset(&mut set).unwrap();
    sigemptyset(&mut oldset).unwrap();

    sigaddset(&mut set, c::SIGUSR2).unwrap();

    pthread_sigmask(c::SIG_BLOCK, Some(&set), Some(&mut oldset)).unwrap();

    assert!(sigismember(&oldset, c::SIGUSR1).unwrap());
    assert!(!sigismember(&oldset, c::SIGUSR2).unwrap());

    raise(c::SIGUSR2).unwrap();

    assert_eq!(sigwait(&set), Ok(c::SIGUSR2));

    raise(c::SIGUSR2).unwrap();

    let mut info = pod_zeroed();
    assert_eq!(sigwaitinfo(&set, Some(&mut info)), Ok(c::SIGUSR2));
    assert_eq!(info.si_signo, c::SIGUSR2);

    raise(c::SIGUSR2).unwrap();

    let timeout = c::timespec {
        tv_sec: 1,
        tv_nsec: 0,
    };

    let mut info = pod_zeroed();
    assert_eq!(
        sigtimedwait(&set, Some(&mut info), &timeout),
        Ok(c::SIGUSR2)
    );
    assert_eq!(info.si_signo, c::SIGUSR2);
}
