use uapi::*;

cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[test]
fn sched_yield_() {
    sched_yield().unwrap();
}
