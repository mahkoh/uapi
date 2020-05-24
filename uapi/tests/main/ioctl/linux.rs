#![allow(non_snake_case)]

#[cfg(any(
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "s390x",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64"
))]
mod test {
    use uapi::*;

    #[test]
    fn IOC() {
        assert_eq!(
            2149668140,
            _IOC(_IOC_READ, c::UINPUT_IOCTL_BASE as _, 44, 33)
        );
    }

    #[test]
    fn IO() {
        assert_eq!(16658, _IO(b'A' as _, 0x12));
    }

    #[test]
    fn IOR() {
        assert_eq!(2147767597, _IOR::<c::c_uint>(c::UINPUT_IOCTL_BASE as _, 45));
    }

    #[test]
    fn IOW() {
        assert_eq!(1074025828, _IOW::<c::c_int>(c::UINPUT_IOCTL_BASE as _, 100));
    }

    #[test]
    fn IOWR() {
        assert_eq!(3221776773, _IOWR::<c::size_t>(b'i' as _, 0x85))
    }

    #[test]
    fn IOC_DIR() {
        assert_eq!(_IOC_READ, _IOC_DIR(_IOC(_IOC_READ, b'a' as _, 65, 22)));
        assert_eq!(_IOC_WRITE, _IOC_DIR(_IOC(_IOC_WRITE, b'a' as _, 65, 22)));
    }

    #[test]
    fn IOC_TYPE() {
        assert_eq!(b'a' as u64, _IOC_TYPE(_IOC(_IOC_READ, b'a' as _, 65, 22)));
    }

    #[test]
    fn IOC_NR() {
        assert_eq!(65, _IOC_NR(_IOC(_IOC_READ, b'a' as _, 65, 22)));
    }

    #[test]
    fn IOC_SIZE() {
        assert_eq!(22, _IOC_SIZE(_IOC(_IOC_READ, b'a' as _, 65, 22)));
    }
}
