/// Forwards its arguments to `format!` and wraps the result in a `Ustring`
#[macro_export]
macro_rules! format_ustr {
    ($($tt:tt)*) => {
        $crate::Ustring::from_string(format!($($tt)*))
    }
}

/// `fn(<string literal>) -> &'static Ustr`
#[macro_export]
macro_rules! ustr {
    ($val:expr) => {
        unsafe { $crate::Ustr::from_bytes_unchecked(concat!($val, "\0").as_bytes()) }
    };
}

/// `fn(<integer type>) -> Result<integer type>`
///
/// The result is `Err` if and only if the argument is `-1`. In this case the return value
/// contains the current value of `ERRNO`.
#[macro_export]
macro_rules! map_err {
    ($expr:expr) => {{
        let val = $expr;
        if val == -1 {
            Err($crate::Errno::default())
        } else {
            Ok(val)
        }
    }};
}

#[cfg(target_os = "linux")]
macro_rules! usize_right_shift {
    ($x:expr) => {{
        const HALF_USIZE: usize = std::mem::size_of::<usize>() / 2;
        (($x) >> HALF_USIZE >> HALF_USIZE)
    }};
}
