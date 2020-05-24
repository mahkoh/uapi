use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod new_linux_api;
        pub use new_linux_api::*;
    }
}
