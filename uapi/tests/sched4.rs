extern crate proc; // https://github.com/rust-lang/rust/issues/64450

#[cfg(target_os = "linux")]
mod wrapper {
    use proc::test_if;
    use std::mem;
    use uapi::*;

    fn test_affinity(pid: c::pid_t) {
        let mut orig = [0; 128];
        sched_getaffinity(pid, &mut orig).unwrap();
        let min_cpu = orig
            .iter()
            .copied()
            .enumerate()
            .filter(|v| v.1 != 0)
            .map(|v| (v.0, v.1.trailing_zeros() as usize))
            .next()
            .unwrap();
        let max_cpu = orig
            .iter()
            .copied()
            .enumerate()
            .rev()
            .filter(|v| v.1 != 0)
            .map(|v| {
                (
                    v.0,
                    mem::size_of_val(&v.1) * 8 - v.1.leading_zeros() as usize - 1,
                )
            })
            .next()
            .unwrap();
        if min_cpu != max_cpu {
            for &cpu in &[min_cpu, max_cpu] {
                let mut buf2 = [0; 128];
                let mut buf3 = buf2;
                buf2[cpu.0] = 1 << cpu.1;
                sched_setaffinity(pid, &buf2).unwrap();
                sched_getaffinity(pid, &mut buf3).unwrap();
                assert_eq!(buf2, buf3);
            }
        }
        sched_setaffinity(pid, &orig).unwrap();
    }

    #[test_if(root)]
    fn test() {
        for &pid in &[0, getpid()] {
            test_affinity(pid);
            for &(policy, nice, priority) in
                &[(c::SCHED_OTHER, 2, 0), (c::SCHED_FIFO, 0, 3)]
            {
                for &flags in &[0, c::SCHED_FLAG_RESET_ON_FORK] {
                    let mut attr: c::sched_attr = pod_zeroed();
                    attr.sched_policy = policy as _;
                    attr.sched_nice = nice;
                    attr.sched_priority = priority;
                    attr.sched_flags = flags as _;
                    sched_setattr(pid, &attr, 0).unwrap();
                    let other = sched_getattr(pid, 0).unwrap();
                    assert_eq!(attr.sched_policy, other.sched_policy);
                    assert_eq!(attr.sched_nice, other.sched_nice);
                    assert_eq!(attr.sched_flags, other.sched_flags);
                    assert_eq!(attr.sched_priority, other.sched_priority);
                }
            }
        }
    }
}
