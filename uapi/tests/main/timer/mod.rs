cfg_if::cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
    }
}
