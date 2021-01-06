use uapi::*;

#[test]
fn test() {
    let pc = c::PRIO_PROCESS as _;
    setpriority(pc, getpid() as _, 1).unwrap();
    let proc = getpriority(pc, getpid() as _).unwrap();
    assert_eq!(proc, 1);
    setpriority(pc, 0, 2).unwrap();
    let proc = getpriority(pc, 0).unwrap();
    assert_eq!(proc, 2);
    let proc = nice(1).unwrap();
    #[cfg(not(target_os = "freebsd"))]
    assert_eq!(proc, 3);
    let proc = getpriority(pc, 0).unwrap();
    assert_eq!(proc, 3);
}
