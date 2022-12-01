This directory contains rust code and equivalent c code together with the assembly they generate and statistics from running llvm-mca on it.

Here is the code in compiler explorer: https://godbolt.org/z/zaeaKKqe9

The Generated Rust code contains an unnecessary jump table with the relevant logic and as a result is ~10% slower than the corresponding c code.
