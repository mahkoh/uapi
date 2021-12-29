use uapi::*;

#[test]
fn timerfd() {
    {
        let e = timerfd_create(c::CLOCK_MONOTONIC, 0).unwrap();
        assert_ne!(fcntl_getfd(*e).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);
    }

    {
        let e = timerfd_create(c::CLOCK_MONOTONIC, c::TFD_CLOEXEC).unwrap();
        assert_eq!(fcntl_getfd(*e).unwrap() & c::FD_CLOEXEC, c::FD_CLOEXEC);

        let time = c::itimerspec {
            it_interval: c::timespec {
                tv_sec: 1000,
                tv_nsec: 2000,
            },
            it_value: c::timespec {
                tv_sec: 3000,
                tv_nsec: 4000,
            },
        };
        let time2 = c::itimerspec {
            it_interval: c::timespec {
                tv_sec: 5000,
                tv_nsec: 6000,
            },
            it_value: c::timespec {
                tv_sec: 7000,
                tv_nsec: 8000,
            },
        };

        timerfd_settime(e.raw(), 0, &time).unwrap();
        let old = timerfd_settime(e.raw(), 0, &time2).unwrap();
        assert_eq!(time.it_interval.tv_sec, old.it_interval.tv_sec);
        assert_eq!(time.it_interval.tv_nsec, old.it_interval.tv_nsec);

        let new = timerfd_gettime(e.raw()).unwrap();
        assert_eq!(time2.it_interval.tv_sec, new.it_interval.tv_sec);
        assert_eq!(time2.it_interval.tv_nsec, new.it_interval.tv_nsec);
    }
}
