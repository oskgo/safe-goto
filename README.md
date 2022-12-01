# safe-goto

Emulating a safe goto-like instruction in Rust according to the spirit of [the "Safe goto with value" pre-RFC from IRLO](https://internals.rust-lang.org/t/pre-rfc-safe-goto-with-value/14470)

The crate contains a single macro, `safe_goto`, which emulates goto with value using a loop wrapping a match on an enum.

## Goals

The purpose of this crate is two-fold:

- Provide a test-bed for experimenting with patterns involving a safe version of goto in Rust
- Highlight areas where the rust compiler has room to improve its branch related optimizations

Non-goals:

- The library does not aim for maximal performance. Doing inline assembly defeats the purpose
- The library does not aim to be a stable foundation for others to build upon. The syntax may change to reflect what might eventually be implemented in the stdlib or as part of the language. Also, the macro is very brittle since it has to handle near-arbitrary rust code

## Examples

The tests and examples contain some examples of the functionality. You can have a look at the expanded macro using tools like [cargo-expand](https://github.com/dtolnay/cargo-expand).
