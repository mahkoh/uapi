# Goals

## Status

2020-05-28 - Active

## Context

The goals of the crate should be clearly documented.

## Resolution

1. The crate should provide wrappers around OS APIs of UNIX-like operating
   systems.
2. The wrappers should be safe if possible.
3. No additional documentation should be required for someone who knows the
   upstream API to understand the wrapper.
4. It should not be necessary to modify the wrapper if upstream extends the API.
5. The wrappers should be as easy to use from Rust as the wrapped APIs are to
   use from C. This includes the verbosity of the code.
6. It should be possible to use the wrappers with 0 overhead.
7. Resources should be managed via RAII.
8. Differences between operating systems should not be unified.

## Consequences

By 3, all wrappers will be available in the crate root.

By 3 and 4, wrapper types will be avoided if possible. By 2, exceptions will be
made if the underlying data structures cannot be used safely.

By 4, there will be no enums in the API. Flag parameters will have the same type
as the underlying parameter.

By 5, wrappers of APIs that accept strings accept all of Rust's string types
such as `str`, `[u8]`, `CStr`, `Path`, `OsStr`, etc. By 6, no allocations must
be performed for `CStr` arguments.

By 5, custom string types will be added to make working with non-UTF-8 and
C-style strings easier.

By 5, traits will be added to make conversions between binary data and structs
easier.

By 7, APIs that return new file descriptors return a wrapper that closes the
file descriptor upon drop.

By 8, APIs that are only available on some operating systems will not be
emulated on other operating systems. For example, macOS has a time API that is
different from the POSIX `clock_*` family of functions.
