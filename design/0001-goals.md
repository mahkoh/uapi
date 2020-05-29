# Goals

## Status

Accepted

### History

2020-05-28 - Accepted

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
