---
output: pdf-document
geometry: margin=2cm
---
Casper Neo \
MPCS 52010 Computer Architecture \
2018 February 2

# Analysis
I did it in a jupyter notebook so you can run the code through jupyter if you have it installed. If not... See `analysis.pdf`. The first part is just code for graphing, then I run the cache emulator a bunch and graph results. I have bullet points describing the results.

# Rust
Installation: https://www.rust-lang.org/en-US/install.html

# Usage
* First change to `cache_emulator` directory
* The emulator
    * To build debug version: `cargo build`
    * To build optimized (release) version: `cargo build --release`
    * To test: `cargo test`
    * To Run: `cargo run`
    * To Run with logs: `RUST_LOG=[LOG LEVEL] ./target/[VERSION]/cache_emulator [FLAGS]`
    * To clean: `cargo clean`
* Analysis
    * To run analysis notebook: `jupyter notebook`, use gui to open `analysis.ipynb`
    * To convert analysis to pdf `jupyter nbconvert --to pdf analysis.ipynb`

# Files
| File | Purpose |
|---------------------|-------------------------------------------------------|
| `Cargo.toml` | cargo uses this file to handle external crates (packages)
| `Cargo.lock` | I have no idea what this does. Cargo did it.
| `src/main.rs` | Entry point to the program: parses flags and runs |
| `src/cli.yml` | Defines the command line flags |
| `src/cpu.rs` | Implementation of `cpu` |
| `src/algorithms.rs` | Implementation of dot product, matrix multiply, etc |
| `analysis.ipynb` | IPython notebook that conducts analysis
