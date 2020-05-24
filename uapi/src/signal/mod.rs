use crate::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(raise(3))]
#[notest]
pub fn raise(sig: c::c_int) -> Result<()> {
    let res = unsafe { c::raise(sig) };
    map_err!(res).map(drop)
}
