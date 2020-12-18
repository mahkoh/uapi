# Unix API

[![crates.io](https://img.shields.io/crates/v/uapi.svg)](http://crates.io/crates/uapi)
[![docs.rs](https://docs.rs/uapi/badge.svg)](http://docs.rs/uapi)

This crate contains safe wrappers around Unix APIs.

## Supported Targets

A target is supported if and only if the crate is tested against it via CI.

The following targets are supported:

- x86_64-unknown-linux-gnu (glibc >= 2.23)
- x86_64-unknown-linux-musl (musl >= 1.1.19)
- x86_64-unknown-freebsd (12)
- x86_64-unknown-openbsd (6.7)
- x86_64-apple-darwin (10.15)

This crate contains little architecture-specific code. Therefore, other
architectures (arm, aarch64, etc.) will probably also work.

## Safety Guarantee

A crate which contains only safe code and which only uses safe APIs from this
crate and libstd is, as a whole, safe.

## Future changes

This crate fully supports reading into uninitialized buffers but the API will
most likely change when the same functionality becomes stable in libstd.

## Comparison with other crates

### libc

This crate builds on the libc crate and uses its declarations of the raw OS APIs
if possible. This crate considers itself to be the next step up from libc: It
safely wraps the raw OS functions but does little beyond that. Integer
parameters are still integer parameters, functions operating on sockets accept
the socket file descriptor as a raw integer, there is no socket wrapper type,
etc.

At the same time, this crate provides the necessary tools to make it as easy to
use from Rust as the raw APIs are to use from C. For example, all of the
following just work:

```rust
open("./file", c::O_RDWR, 0);
open(b"./file", c::O_RDWR, 0);
open(CStr::from_ptr(p), c::O_RDWR, 0);
open(Path::new("./file"), c::O_RDWR, 0);
```

See the crate documentation for more details.

### nix

- **nix** uses a nested module structure. **uapi** exports all APIs in the crate
  root.
- **nix** uses enums and bitflags for integer/flag parameters. **uapi** uses
  plain integers.
- **nix** uses methods declared on wrapper types to expose APIs. **uapi** uses
  free functions unless doing so would be unsafe.
- **nix** uses enums for the values produced and consumed by certain generic OS
  APIs (e.g. control messages.) **uapi** uses generic functions that do not
  restrict the types that can be used.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
