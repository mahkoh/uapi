# Unix API

This crate contains safe wrappers around Unix APIs.

## Supported Targets

A target is supported if and only if the crate is tested against it via CI.

The following targets are supported:

- x86_64-unknown-linux-gnu (glibc >= 2.23)
- x86_64-unknown-linux-musl (musl >= 1.1.19)
- x86_64-unknown-freebsd (12)
- x86_64-unknown-openbsd (6.7)
- x86_64-apple-darwin (10.15)

This crate contains little architecture-specific code. Therefore, other architectures
(arm, aarch64, etc.) will probably also work.

## Safety Guarantee

A crate which contains only safe code and which only uses safe APIs from this crate and
libstd is, as a whole, safe.

## License

This project is licensed under either of

- Apache License, Version 2.0
- MIT License

at your option.
