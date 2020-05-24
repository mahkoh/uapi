use crate::c;

/// Gets a pointer to the current thread's errno
pub fn errno_location() -> *mut c::c_int {
    #[cfg(any(target_os = "dragonfly", target_os = "linux"))]
    unsafe {
        c::__errno_location()
    }

    #[cfg(any(target_os = "android", target_os = "netbsd", target_os = "openbsd"))]
    unsafe {
        c::__errno()
    }

    #[cfg(any(target_os = "freebsd", target_os = "ios", target_os = "macos"))]
    unsafe {
        c::__error()
    }
}

/// Gets the current thread's errno
pub fn get_errno() -> c::c_int {
    unsafe { *errno_location() }
}

/// Sets the current thread's errno
pub fn set_errno(val: c::c_int) {
    unsafe { *errno_location() = val };
}
