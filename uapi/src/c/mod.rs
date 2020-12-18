//! Re-export of the libc crate with missing items added
//!
//! Items should be upstreamed if possible.

pub use libc::*;

use cfg_if::cfg_if;

#[cfg(target_os = "dragonfly")]
extern "C" {
    pub fn __errno_location() -> *mut c_int;
}

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::sockaddr_nl;
        pub use linux::*;
    }
}
