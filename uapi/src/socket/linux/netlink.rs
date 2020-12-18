use crate::{socket::linux::netlink::sealed::Sealed, *};
use proc::{beta};
use std::{
    convert::{TryFrom, TryInto},
    mem,
    mem::MaybeUninit,
};

const ALIGN: usize = 4 - 1;

/// Increases the size to the next multiple of 4
///
/// See also the crate documentation.
#[beta]
pub fn nlmsg_align(size: usize) -> usize {
    (size + ALIGN) & !ALIGN
}

/// Reads an object from an netlink message buffer
///
/// Note that this function will advance the buffer by the size of `T` rounded up to the
/// next multiple of 4.
///
/// See also the crate documentation.
#[beta]
pub fn nlmsg_read<T: Pod>(buf: &mut &[u8]) -> Result<(usize, T)> {
    let object_size = mem::size_of::<T>();
    if buf.len() < object_size {
        return einval();
    }
    let mut obj = MaybeUninit::<T>::uninit();
    unsafe {
        std::ptr::copy_nonoverlapping(
            buf.as_ptr(),
            obj.as_mut_ptr() as *mut u8,
            object_size,
        );
    }
    let space = nlmsg_align(object_size).min(buf.len());
    *buf = &buf[space..];
    let obj = unsafe { obj.assume_init() };
    Ok((space, obj))
}

/// The general shape of a header of a netlink message
///
/// Note that this is not the same as [c::nlmsghdr].
///
/// See also the crate documentation.
#[beta]
pub trait NlmsgHeader: Sized {
    /// Returns the length of the padded header + payload
    ///
    /// This function returns an error if the length cannot be converted to `usize`
    /// without truncation.
    fn len(&self) -> Result<usize>;
    /// Sets the length of the padded header + payload
    ///
    /// This function returns an error if the length cannot be converted to the internal
    /// length type without truncation.
    fn set_len(&mut self, len: usize) -> Result<()>;
}

mod sealed {
    pub trait Sealed {}
}

/// Extension trait for netlink message headers
///
/// See also the crate documentation.
#[beta]
pub trait NlmsgHeaderExt: NlmsgHeader + Sealed {
    /// Reads a header plus payload from a netlink message
    ///
    /// Returns the space consumed, the header, and the payload.
    fn read<'a>(buf: &mut &'a [u8]) -> Result<(usize, Self, &'a [u8])>
    where
        Self: Pod,
    {
        nlmsg_read_header(buf)
    }
}

impl<T: NlmsgHeader> Sealed for T {
}
impl<T: NlmsgHeader> NlmsgHeaderExt for T {
}

impl NlmsgHeader for () {
    fn len(&self) -> Result<usize> {
        Ok(0)
    }

    fn set_len(&mut self, _len: usize) -> Result<()> {
        Ok(())
    }
}

macro_rules! nlh {
    ($ty:ident, $field:ident) => {
        impl NlmsgHeader for c::$ty {
            fn len(&self) -> Result<usize> {
                usize::try_from(self.$field).or_else(|_| einval())
            }

            fn set_len(&mut self, len: usize) -> Result<()> {
                self.$field = match len.try_into() {
                    Ok(v) => v,
                    Err(_) => return einval(),
                };
                Ok(())
            }
        }
    };
}

nlh!(nlmsghdr, nlmsg_len);
nlh!(nlattr, nla_len);

fn nlmsg_read_header<'a, H: Pod + NlmsgHeader>(
    buf: &mut &'a [u8],
) -> Result<(usize, H, &'a [u8])> {
    let header_space = nlmsg_align(mem::size_of::<H>());
    let hdr: H = {
        let mut buf = *buf;
        nlmsg_read(&mut buf)?.1
    };
    let len = hdr.len()?;
    if len < header_space {
        return einval();
    }
    if buf.len() < len {
        return einval();
    }
    if usize::max_value() - len < ALIGN {
        return einval();
    }
    let space = nlmsg_align(len).min(buf.len());
    let data = &buf[header_space..len];
    *buf = &buf[space..];
    Ok((space, hdr, data))
}

/// A writer for netlink messages
///
/// See also the crate documentation.
#[beta]
pub struct NlmsgWriter<'a, H: NlmsgHeader = ()> {
    buf: &'a mut [u8],
    header: H,
    len: usize,
    parent_len: Option<&'a mut usize>,
}

impl<'a, H: NlmsgHeader> NlmsgWriter<'a, H> {
    /// Creates a new writer that uses the buffer as backing storage
    pub fn new(buf: &'a mut [u8], header: H) -> Result<Self> {
        Self::new2(buf, None, header)
    }

    fn new2<'b, H2: NlmsgHeader>(
        buf: &'b mut [u8],
        parent_len: Option<&'b mut usize>,
        header: H2,
    ) -> Result<NlmsgWriter<'b, H2>> {
        let size = mem::size_of::<H2>();
        if buf.len() < size {
            return einval();
        }
        Ok(NlmsgWriter {
            buf,
            header,
            len: size,
            parent_len,
        })
    }

    /// Writes an object to the buffer
    ///
    /// This involves three steps:
    ///
    /// - The write position is aligned to the next 4 byte boundary
    /// - The object is written
    /// - The write position is advanced by the size of the object
    ///
    /// Returns an error if the buffer does not contain enough space.
    pub fn write<T: ?Sized>(&mut self, data: &T) -> Result<()> {
        let aligned_len = nlmsg_align(self.len);
        {
            if aligned_len > self.buf.len() {
                return einval();
            }
            let buf = &mut self.buf[aligned_len..];
            let data_size = mem::size_of_val(data);
            if buf.len() < data_size {
                return einval();
            }
            unsafe {
                let ptr = buf.as_mut_ptr();
                ptr.copy_from_nonoverlapping(data as *const _ as *const _, data_size);
                black_box(ptr);
            }
        }
        self.len = aligned_len + mem::size_of_val(data);
        Ok(())
    }

    /// Nests a new message within the write buffer
    ///
    /// Returns an error if there is not enough space to write the header at the next
    /// 4 byte boundary.
    ///
    /// When the nested writer is dropped, the behavior is as if the nested message
    /// had been written to a separate buffer and then written to this writer using
    /// [`Self::write`].
    pub fn nest<H2: NlmsgHeader>(&mut self, header: H2) -> Result<NlmsgWriter<H2>> {
        let aligned_len = nlmsg_align(self.len);
        if aligned_len >= self.buf.len() {
            return einval();
        }
        Self::new2(&mut self.buf[aligned_len..], Some(&mut self.len), header)
    }

    fn finalize_mut(&mut self) -> Result<usize> {
        self.header.set_len(self.len)?;
        let ptr = self.buf.as_mut_ptr();
        unsafe {
            ptr.copy_from_nonoverlapping(
                &self.header as *const _ as *const _,
                mem::size_of::<H>(),
            );
            black_box(ptr);
        }
        if let Some(parent_len) = &mut self.parent_len {
            **parent_len = nlmsg_align(**parent_len) + self.len;
        }
        Ok(self.len)
    }

    /// Sets the length field of the header to the correct value
    ///
    /// This function returns an error if [`NlmsgHeader::set_len]` fails.
    pub fn finalize(mut self) -> Result<usize> {
        let len = self.finalize_mut()?;
        mem::forget(self);
        Ok(len)
    }
}

impl<'a, H: NlmsgHeader> Drop for NlmsgWriter<'a, H> {
    fn drop(&mut self) {
        self.finalize_mut().expect("could not finalize header");
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_client_to_client() -> Result<()> {
        let s1 = socket(c::AF_NETLINK, c::SOCK_RAW, c::NETLINK_USERSOCK)?;
        let s2 = socket(c::AF_NETLINK, c::SOCK_RAW, c::NETLINK_USERSOCK)?;
        let mut addr = c::sockaddr_nl {
            nl_family: c::AF_NETLINK as _,
            nl_pad: 0,
            nl_pid: 0,
            nl_groups: 0,
        };
        bind(*s1, &addr)?;
        getsockname(*s1, &mut addr)?;
        let mut buf = [0; 128];
        let mut writer = NlmsgWriter::new(
            &mut buf,
            c::nlmsghdr {
                nlmsg_len: 0,
                nlmsg_type: 1,
                nlmsg_flags: 2,
                nlmsg_seq: 3,
                nlmsg_pid: 4,
            },
        )?;
        {
            let mut attr = writer.nest(c::nlattr {
                nla_len: 0,
                nla_type: 5,
            })?;
            {
                let mut attr = attr.nest(c::nlattr {
                    nla_len: 0,
                    nla_type: 6,
                })?;
                attr.write(&1u8)?;
            }
            {
                let mut attr = attr.nest(c::nlattr {
                    nla_len: 0,
                    nla_type: 7,
                })?;
                attr.write("hello world")?;
            }
        }
        let size = writer.finalize()?;
        sendto(*s2, &buf[..size], 0, &addr)?;
        let len = recv(*s1, &mut buf[..], 0)?;
        let mut reader = &buf[..len];
        let (_, nlmsghdr, mut payload) = c::nlmsghdr::read(&mut reader)?;
        assert_eq!(nlmsghdr.nlmsg_type, 1);
        assert_eq!(nlmsghdr.nlmsg_flags, 2);
        assert_eq!(nlmsghdr.nlmsg_seq, 3);
        assert_eq!(nlmsghdr.nlmsg_pid, 4);
        {
            let (_, outer_attr, mut payload) = c::nlattr::read(&mut payload)?;
            assert_eq!(outer_attr.nla_type, 5);
            {
                let (_, inner_attr, payload) = c::nlattr::read(&mut payload)?;
                assert_eq!(inner_attr.nla_type, 6);
                assert_eq!(pod_read::<u8, _>(payload)?, 1);
            }
            {
                let (_, inner_attr, payload) = c::nlattr::read(&mut payload)?;
                assert_eq!(inner_attr.nla_type, 7);
                assert_eq!(payload, b"hello world");
            }
            assert!(payload.is_empty());
        }
        assert!(payload.is_empty());
        assert!(reader.is_empty());
        Ok(())
    }

    #[test]
    fn test_rt_netlink() -> Result<()> {
        testutils::strace(true, || {
            let socket = socket(c::AF_NETLINK, c::SOCK_RAW, c::NETLINK_ROUTE)?;
            let addr = c::sockaddr_nl {
                nl_family: c::AF_NETLINK as _,
                nl_pad: 0,
                nl_pid: 0,
                nl_groups: 0,
            };
            bind(*socket, &addr)?;
            let mut buf = [0; 32 * 1024];
            let mut writer = NlmsgWriter::new(
                &mut buf,
                c::nlmsghdr {
                    nlmsg_len: 0,
                    nlmsg_type: c::RTM_GETLINK,
                    nlmsg_flags: (c::NLM_F_REQUEST | c::NLM_F_DUMP) as _,
                    nlmsg_seq: 0,
                    nlmsg_pid: 0,
                },
            )?;
            writer.write(&c::ifinfomsg {
                ifi_family: c::AF_PACKET as _,
                ifi_type: 0,
                ifi_index: 0,
                ifi_flags: 0,
                ifi_change: 0,
            })?;
            {
                let mut attr = writer.nest(c::nlattr {
                    nla_len: 0,
                    nla_type: c::IFLA_EXT_MASK,
                })?;
                attr.write(&1u32)?;
            }
            let size = writer.finalize()?;
            send(*socket, &buf[..size], 0)?;
            let mut found_loopback = false;
            'outer: loop {
                let len = recv(*socket, &mut buf[..], c::MSG_TRUNC)?;
                let mut reader = &buf[..len];
                while reader.len() > 0 {
                    let (_, header, mut payload) = c::nlmsghdr::read(&mut reader)?;
                    if header.nlmsg_type == c::NLMSG_DONE as _ {
                        break 'outer;
                    }
                    assert_eq!(header.nlmsg_type, c::RTM_NEWLINK);
                    let (_, ifi) = nlmsg_read::<c::ifinfomsg>(&mut payload)?;
                    let is_loopback = ifi.ifi_type == c::ARPHRD_LOOPBACK;
                    if is_loopback {
                        found_loopback = true;
                        assert_eq!(ifi.ifi_family, c::AF_UNSPEC as c::c_uchar);
                        assert_ne!(ifi.ifi_flags & c::IFF_UP as c::c_uint, 0);
                        assert_ne!(ifi.ifi_flags & c::IFF_LOOPBACK as c::c_uint, 0);
                    }
                    let mut found_name = false;
                    while payload.len() > 0 {
                        let (_, header, payload) = c::nlattr::read(&mut payload)?;
                        if header.nla_type == c::IFLA_IFNAME {
                            found_name = true;
                            if is_loopback {
                                assert_eq!(payload, b"lo\0");
                            }
                        }
                    }
                    assert!(found_name);
                    if header.nlmsg_flags & c::NLM_F_MULTI as u16 == 0 {
                        break 'outer;
                    }
                }
            }
            assert!(found_loopback);
            Ok(())
        })
    }
}
