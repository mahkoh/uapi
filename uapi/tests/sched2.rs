extern crate proc; // https://github.com/rust-lang/rust/issues/64450

#[cfg(target_os = "linux")]
mod wrapper {
    use proc::test_if;
    use uapi::*;

    #[test_if(root)]
    fn test_setsched() {
        for &sched in &[c::SCHED_FIFO, c::SCHED_RR] {
            for &pid in &[0, getpid()] {
                for i in 1..3 {
                    let mut param: c::sched_param = pod_zeroed();
                    param.sched_priority = i;
                    sched_setscheduler(pid, sched, &param).unwrap();
                    assert_eq!(sched_getscheduler(pid).unwrap(), sched);
                    let mut param = sched_getparam(pid).unwrap();
                    assert_eq!(param.sched_priority, i);
                    param.sched_priority += 10;
                    sched_setparam(pid, &param).unwrap();
                    let param = sched_getparam(pid).unwrap();
                    assert_eq!(param.sched_priority, i + 10);
                }
            }
        }
    }
}
