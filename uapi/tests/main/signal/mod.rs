use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[test]
fn sigset() {
    let mut set = empty_sig_set().unwrap();

    assert!(!sigismember(&set, c::SIGUSR1).unwrap());
    assert!(!sigismember(&set, c::SIGUSR2).unwrap());

    sigfillset(&mut set).unwrap();

    assert!(sigismember(&set, c::SIGUSR1).unwrap());
    assert!(sigismember(&set, c::SIGUSR2).unwrap());

    sigemptyset(&mut set).unwrap();

    assert!(!sigismember(&set, c::SIGUSR1).unwrap());
    assert!(!sigismember(&set, c::SIGUSR2).unwrap());

    sigaddset(&mut set, c::SIGUSR1).unwrap();

    assert!(sigismember(&set, c::SIGUSR1).unwrap());
    assert!(!sigismember(&set, c::SIGUSR2).unwrap());

    sigaddset(&mut set, c::SIGUSR2).unwrap();

    assert!(sigismember(&set, c::SIGUSR1).unwrap());
    assert!(sigismember(&set, c::SIGUSR2).unwrap());

    sigdelset(&mut set, c::SIGUSR1).unwrap();

    assert!(!sigismember(&set, c::SIGUSR1).unwrap());
    assert!(sigismember(&set, c::SIGUSR2).unwrap());

    sigdelset(&mut set, c::SIGUSR2).unwrap();

    assert!(!sigismember(&set, c::SIGUSR1).unwrap());
    assert!(!sigismember(&set, c::SIGUSR2).unwrap());
}
