---
output: pdf-document
geometry: margin=2cm
---
Casper Neo \
MPCS 52010 Computer Architecture \
2018 February 2

# Analysis
<!-- TODO -->

# Rust
Installation: https://www.rust-lang.org/en-US/install.html

# Usage
* First change to `cache_emulator` directory
* To build: `cargo build`
* To test: `cargo test`
* To Run: `cargo run`
* To Run with logs: `RUST_LOG=[LOG LEVEL] ./target/debug/cache_emulator [FLAGS]`

# Files
| File | Purpose |
|---------------------|-------------------------------------------------------|
| `Cargo.toml` | cargo uses this file to handle external crates (packages)
| `Cargo.lock` | I have no idea what this does. Cargo did it.
| `src/main.rs` | Entry point to the program: parses flags and runs |
| `src/cli.yml` | Defines the command line flags |
| `src/cpu.rs` | Implementation of `cpu` (tests of private functions on the bottom) |
| `src/algorithms.rs` | Implementation of dot product, matrix multiply, etc |
| `src/test.rs` | Tests the CPU and algorithms
