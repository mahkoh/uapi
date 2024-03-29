use crate::*;
use std::{convert::TryFrom, ffi::CStr, mem, mem::MaybeUninit, ops::Deref, ptr};

#[man(copy_file_range(2))]
pub fn copy_file_range(
    fd_in: c::c_int,
    off_in: Option<&mut c::loff_t>,
    fd_out: c::c_int,
    off_out: Option<&mut c::loff_t>,
    len: c::size_t,
    flags: c::c_uint,
) -> Result<usize> {
    let val = unsafe {
        c::syscall(
            c::SYS_copy_file_range,
            fd_in as usize,
            off_in.map(|p| p as *mut _).unwrap_or(ptr::null_mut()) as usize,
            fd_out as usize,
            off_out.map(|p| p as *mut _).unwrap_or(ptr::null_mut()) as usize,
            len,
            flags as usize,
        )
    };
    map_err!(val).map(|v| v as _)
}

#[man(renameat2(2))]
pub fn renameat2<'a, 'b>(
    olddirfd: c::c_int,
    oldpath: impl IntoUstr<'a>,
    newdirfd: c::c_int,
    newpath: impl IntoUstr<'b>,
    flags: c::c_uint,
) -> Result<()> {
    let oldpath = oldpath.into_ustr();
    let newpath = newpath.into_ustr();
    let val = unsafe {
        c::syscall(
            c::SYS_renameat2,
            olddirfd as usize,
            oldpath.as_ptr() as usize,
            newdirfd as usize,
            newpath.as_ptr() as usize,
            flags as usize,
        )
    };
    map_err!(val).map(drop)
}

#[man(splice(2))]
pub fn splice(
    fd_in: c::c_int,
    off_in: Option<&mut c::loff_t>,
    fd_out: c::c_int,
    off_out: Option<&mut c::loff_t>,
    len: c::size_t,
    flags: c::c_uint,
) -> Result<usize> {
    let val = unsafe {
        c::splice(
            fd_in,
            off_in.map(|p| p as *mut _).unwrap_or(ptr::null_mut()),
            fd_out,
            off_out.map(|p| p as *mut _).unwrap_or(ptr::null_mut()),
            len,
            flags,
        )
    };
    map_err!(val).map(|v| v as _)
}

#[man(tee(2))]
pub fn tee(
    fd_in: c::c_int,
    fd_out: c::c_int,
    len: c::size_t,
    flags: c::c_uint,
) -> Result<usize> {
    let val = unsafe { c::tee(fd_in, fd_out, len, flags) };
    map_err!(val).map(|v| v as _)
}

#[man(inotify_init1(2))]
pub fn inotify_init1(flags: c::c_int) -> Result<OwnedFd> {
    let val = unsafe { c::inotify_init1(flags) };
    map_err!(val).map(OwnedFd::new)
}

#[man(inotify_add_watch(2))]
pub fn inotify_add_watch<'a>(
    fd: c::c_int,
    pathname: impl IntoUstr<'a>,
    mask: u32,
) -> Result<c::c_int> {
    let pathname = pathname.into_ustr();
    let val = unsafe { c::inotify_add_watch(fd, pathname.as_ptr(), mask) };
    map_err!(val)
}

#[man(inotify_rm_watch(2))]
pub fn inotify_rm_watch(fd: c::c_int, wd: c::c_int) -> Result<()> {
    let val = unsafe { c::inotify_rm_watch(fd, wd) };
    map_err!(val).map(drop)
}

/// Reads from an inotify file descriptor and returns an iterator over the results
pub fn inotify_read<T: Pod + ?Sized>(
    fd: c::c_int,
    buf: &mut T,
) -> Result<impl IntoIterator<Item = InotifyEvent>> {
    let res = read(fd, buf)?;
    Ok(InotifyIter(res))
}

struct InotifyIter<'a>(&'a [u8]);

impl<'a> Iterator for InotifyIter<'a> {
    type Item = InotifyEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        const SIZE: usize = mem::size_of::<c::inotify_event>();

        if self.0.is_empty() {
            return None;
        }
        if self.0.len() < SIZE {
            panic!("short inotify read");
        }
        unsafe {
            // prepare an aligned instance of event
            #[repr(C)]
            union EventBuf {
                event: c::inotify_event,
                buf: [u8; SIZE],
            }
            let mut event = EventBuf { buf: [0; SIZE] };
            event.buf.copy_from_slice(&self.0[..SIZE]);

            // validate the name
            let name_start: *const c::c_char =
                (self.0 as *const _ as *const c::c_char).add(SIZE);
            let name_len = event.event.len as usize;
            let total_size = SIZE
                .checked_add(name_len)
                .expect("overflowing inotify event length");
            if self.0.len() < total_size {
                panic!("short inotify read");
            }
            if name_len > 0 && *name_start.add(name_len - 1) != 0 {
                panic!("inotify event has no terminating nul byte")
            }

            // advance the buffer
            self.0 = &self.0[total_size..];

            // return the event
            Some(InotifyEvent {
                event: event.event,
                name: if name_len > 0 {
                    CStr::from_ptr(name_start)
                } else {
                    Ustr::empty().as_c_str().unwrap()
                },
            })
        }
    }
}

/// Wrapper for `libc::inotify_event`
pub struct InotifyEvent<'a> {
    event: c::inotify_event,
    name: &'a CStr,
}

impl<'a> InotifyEvent<'a> {
    /// Returns the `name` field of the event
    pub fn name(&self) -> &'a CStr {
        self.name
    }
}

impl Deref for InotifyEvent<'_> {
    type Target = c::inotify_event;

    fn deref(&self) -> &Self::Target {
        &self.event
    }
}

#[man(sendfile(2))]
pub fn sendfile(
    out_fd: c::c_int,
    in_fd: c::c_int,
    offset: Option<&mut c::off_t>,
    size: c::size_t,
) -> Result<usize> {
    let val = unsafe {
        c::sendfile(
            out_fd,
            in_fd,
            offset.map(|o| o as *mut _).unwrap_or(ptr::null_mut()),
            size,
        )
    };
    map_err!(val).map(|v| v as _)
}

#[man(major(3))]
pub const fn major(dev: c::dev_t) -> u64 {
    ((dev >> 32) & 0xffff_f000) | ((dev >> 8) & 0xfff)
}

#[man(minor(3))]
pub const fn minor(dev: c::dev_t) -> u64 {
    ((dev >> 12) & 0xffff_ff00) | (dev & 0xff)
}

#[man(makedev(3))]
pub const fn makedev(major: u64, minor: u64) -> c::dev_t {
    ((major & 0xffff_f000) << 32)
        | ((major & 0xfff) << 8)
        | ((minor & 0xffff_ff00) << 12)
        | (minor & 0xff)
}

#[man(statfs(2))]
pub fn statfs<'a>(path: impl IntoUstr<'a>) -> Result<c::statfs> {
    let path = path.into_ustr();
    let mut statfs = MaybeUninit::uninit();
    let val = unsafe { c::statfs(path.as_ptr(), statfs.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { statfs.assume_init() })
}

#[man(fstatfs(2))]
pub fn fstatfs(fd: c::c_int) -> Result<c::statfs> {
    let mut statfs = MaybeUninit::uninit();
    let val = unsafe { c::fstatfs(fd, statfs.as_mut_ptr()) };
    map_err!(val).map(|_| unsafe { statfs.assume_init() })
}

#[man(preadv2(2))]
pub fn preadv2<T: MaybeUninitIovecMut + ?Sized>(
    fd: c::c_int,
    bufs: &mut T,
    offset: c::loff_t,
    flags: c::c_int,
) -> Result<InitializedIovec> {
    let bufs = unsafe { bufs.as_iovec_mut() };
    let len = i32::try_from(bufs.len()).unwrap_or(i32::max_value());
    let val = unsafe {
        c::syscall(
            c::SYS_preadv2,
            fd as usize,
            bufs.as_mut_ptr() as *mut c::iovec as usize,
            len as usize,
            offset as usize,
            usize_right_shift!(offset) as usize,
            flags as usize,
        )
    };
    let val = map_err!(val)? as usize;
    unsafe { Ok(InitializedIovec::new(bufs, val)) }
}

#[man(pwritev2(2))]
pub fn pwritev2<T: MaybeUninitIovec + ?Sized>(
    fd: c::c_int,
    bufs: &T,
    offset: c::loff_t,
    flags: c::c_int,
) -> Result<usize> {
    let bufs = bufs.as_iovec();
    let len = i32::try_from(bufs.len()).unwrap_or(i32::max_value());
    let val = unsafe {
        c::syscall(
            c::SYS_pwritev2,
            fd as usize,
            black_box_id(bufs.as_ptr()) as *const c::iovec as usize,
            len as usize,
            offset as usize,
            usize_right_shift!(offset) as usize,
            flags as usize,
        )
    };
    map_err!(val).map(|v| v as usize)
}

#[man(dup3(2))]
pub fn dup3(old: c::c_int, new: c::c_int, flags: c::c_int) -> Result<c::c_int> {
    let res = unsafe { c::dup3(old, new, flags) };
    map_err!(res)
}

#[man(fallocate(2))]
pub fn fallocate(
    fd: c::c_int,
    mode: c::c_int,
    offset: c::off_t,
    len: c::off_t,
) -> Result<()> {
    let val = unsafe { c::fallocate(fd, mode, offset, len) };
    map_err!(val).map(drop)
}

#[man(openat2(2))]
pub fn openat2<'a>(
    dirfd: c::c_int,
    pathname: impl IntoUstr<'a>,
    how: &c::open_how,
) -> Result<OwnedFd> {
    let path = pathname.into_ustr();
    let mut how = *how;
    let val =
        unsafe { c::openat2(dirfd, path.as_ptr(), &mut how, mem::size_of_val(&how)) };
    map_err!(val).map(OwnedFd::new)
}

#[man(close_range(2))]
pub fn close_range(first: c::c_uint, last: c::c_uint, flags: c::c_uint) -> Result<()> {
    let val = unsafe {
        c::syscall(
            c::SYS_close_range,
            first as usize,
            last as usize,
            flags as usize,
        )
    };
    map_err!(val).map(drop)
}
