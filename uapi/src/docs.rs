//! Module containing the crate documentation
//!
//! This crate provides safe wrappers for Unix APIs.
//!
//! All wrappers are provided in the crate root. There are no wrapper types unless the raw
//! types cannot be used safely and as noted below.
//!
//! # File Descriptors
//!
//! This crate implements lifecycle management of file descriptors with two types:
//!
//! - `OwnedFd` - An owned file descriptor
//! - `Fd` - Any kind of file descriptor
//!
//! `OwnedFd` is distinct from `Fd` in that it assumes ownership of the contained file
//! descriptor. It implements `Drop` and can be converted from and to many libstd types.
//!
//! Both `OwnedFd` and `Fd` implement `Read` and `Write`, and `Deref` to `libc::c_int`.
//!
//! APIs that return owned file descriptors return `OwnedFd`. APIs that assume ownership
//! take `OwnedFd`.
//!
//! It's important to recognize that `OwnedFd` is not about safety but about convenience.
//! An `OwnedFd` can be created from any integer by calling `OwnedFd::new`. An `OwnedFd`
//! can be unwrapped by calling `OwnedFd::unwrap`.
//!
//! # Strings
//!
//! This crate contains 3 string types:
//!
//! - `&Bstr` - Any sequence of bytes
//! - `&Ustr` - Any sequence of bytes with a terminating nul byte
//! - `Ustring` - Owned version of `&Ustr`
//!
//! And several associated traits:
//!
//! - `AsUstr` - Cheap conversion into `&Ustr`
//! - `Bytes` - Types with a binary representation
//! - `IntoUstr` - Types that can be converted into `Cov<'a, Ustr>`
//!
//! `&Bstr` is a simple wrapper around `&[u8]` which supports conversions from and to many
//! libstd string types:
//!
//! - `&[u8]`
//! - `&str`
//! - `&Path`
//! - `&OsStr`
//! - `&CStr`
//!
//! `&Bstr` supports `Debug` and `Display`. The implementations are the same as the ones
//! used for `&Path`. To access the `Display` implementation, call `Bstr::display()`.
//! `&Bstr` supports `PartialEq` for many libstd types, `&Ustr`, and `Ustring`.
//!
//! `&Ustr` is like `&Bstr` but guarantees that there is a nul byte after the last element
//! of the sequence. Unlike `&CStr`, `&Ustr` can contain inner nul bytes. `&Ustr` derefs
//! to `&Bstr`. Like `&Bstr`, `&Ustr` can be converted from and to many libstd types.
//!
//! `Ustring` is the owned version of `&Ustr`. It supports conversion from and to many
//! owned libstd string types.
//!
//! APIs that return strings usually return `&CStr`. Since `&CStr` is uncomfortable to
//! work with, `AsUstr` provides the `as_ustr` method on `&CStr`.
//!
//! APIs that accept strings usually accept `impl IntoUstr`. `IntoUstr` is implemented for
//! many libstd string types, `&Bstr`, `&Ustr`, and `Ustring`. The implementations of
//! `IntoUstr` perform least-cost conversions into `&Ustr`. For example
//!
//! ```rust,ignore
//! fn f<'a>(s: impl IntoUstr<'a>) {
//!     let _ = s.into_ustr();
//! }
//!
//! f("abc"); // allocates
//! f("abc\0"); // does not allocate
//! f("abc".to_string()); // appends a nul byte to the `String`
//! f(CStr::from_ptr(p)); // does not allocate
//! ```
//!
//! # Pod & Packed
//!
//! This crate contains two traits for conversions of data structures from and to bytes:
//!
//! - `Pod` - Plain Old Data
//! - `Packed` - Types without padding bytes
//!
//! The central property of `Pod` types is that they can be converted from bytes without
//! first having to validate the bytes. All C types are `Pod`, though this crate only
//! implements `Pod` for a selected number of types. More implementations might get added
//! in the future.
//!
//! A `Packed` type does not contain padding bytes. This means that they can be safely
//! converted into bytes because all bytes are guaranteed to be initialized.
//!
//! These types are useful for working with APIs that transfer structures over
//! byte-oriented APIs like cmsg, inotify, signalfd, etc.
//!
//! The utilities to facilitate this are
//!
//! - `pod_zeroed` - Returns an instance of a `Pod` type with all bytes zeroed.
//! - `pod_read` - Reads an instance of a `Packed` type as an instance of a `Pod` type.
//! - `pod_iter` - Iterates over the instances of a `Pod` typed stored in an instance of a
//!    `Packed` type.
//! - `pod_read_init` - Reads an initial part of an instance of a `Packed` type as an
//!     instance of a `Pod` type.
//! - `pod_write` - Writes an instance of a `Packed` type to an instance of a `Pod` type.
//! - `as_bytes` - Returns the bytes of an instance of a `Packed` type.
//!
//! If a certain type does not implement `Pod` or `Packed` and you require it, you can
//! use `assert_pod` or `assert_packed`.
//!
//! # Control Messages (cmsg)
//!
//! This crate provides safe functions for writing and reading control messages:
//!
//! - `cmsg_write` - Writes a cmsg to a byte buffer
//! - `cmsg_read` - Reads a cmsg from a byte buffer
