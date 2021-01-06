extern crate proc; // https://github.com/rust-lang/rust/issues/64450

#[cfg(target_os = "linux")]
mod wrapper {
    use proc::test_if;
    use uapi::*;

    #[test_if(root)]
    fn test_rr() {
        let mut param: c::sched_param = pod_zeroed();
        param.sched_priority = 1;
        sched_setscheduler(0, c::SCHED_RR, &param).unwrap();
        let iv = sched_rr_get_interval(0).unwrap();
        assert!(iv.tv_sec > 0 || iv.tv_nsec > 0)
    }
}
