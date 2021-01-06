use uapi::*;

#[test]
fn sched_get_priority() {
    let min = sched_get_priority_min(c::SCHED_FIFO).unwrap();
    let max = sched_get_priority_max(c::SCHED_FIFO).unwrap();
    assert!(max - min + 1 >= 32);
}
