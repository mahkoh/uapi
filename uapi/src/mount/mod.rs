use crate::*;
use cfg_if::cfg_if;
use std::ptr;

cfg_if! {
    if #[cfg(target_os = "linux")] {
        mod linux;
        pub use linux::*;
    }
}

#[man(mount(2))]
#[notest]
pub fn mount<'a, 'b, 'c, 'd>(
    src: impl IntoUstr<'a>,
    target: impl IntoUstr<'b>,
    fstype: impl IntoUstr<'c>,
    flags: c::c_ulong,
    data: Option<&'d [u8]>,
) -> Result<()> {
    let src = src.into_ustr();
    let target = target.into_ustr();
    let fstype = fstype.into_ustr();
    let res = unsafe {
        c::mount(
            src.as_ptr(),
            target.as_ptr(),
            fstype.as_ptr(),
            flags,
            data.map(|d| d.as_ptr() as *const _).unwrap_or(ptr::null()),
        )
    };
    map_err!(res).map(drop)
}

#[man(umount(2))]
#[notest]
pub fn umount<'a>(target: impl IntoUstr<'a>) -> Result<()> {
    let target = target.into_ustr();
    let res = unsafe { c::umount(target.as_ptr()) };
    map_err!(res).map(drop)
}

#[cfg(any(target_os = "linux", target_os = "android"))]
#[man(umount2(2))]
#[notest]
pub fn umount2<'a>(target: impl IntoUstr<'a>, flags: c::c_int) -> Result<()> {
    let target = target.into_ustr();
    let res = unsafe { c::umount2(target.as_ptr(), flags) };
    map_err!(res).map(drop)
}
