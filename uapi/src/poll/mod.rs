use crate::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(poll(2))]
#[notest]
pub fn poll(fds: &mut [c::pollfd], timeout: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::poll(fds.as_mut_ptr(), fds.len() as _, timeout) };
    map_err!(res)
}
