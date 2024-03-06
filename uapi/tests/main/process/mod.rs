use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
    }
}

#[test]
fn clock() {
    let mut ts = pod_zeroed();
    clock_gettime(c::CLOCK_MONOTONIC, &mut ts).unwrap();
    clock_getres(c::CLOCK_MONOTONIC, &mut ts).unwrap();
}
