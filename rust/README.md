# Arbitrary-Distribution Enumerative Sphere Shaping (AD-ESS)

This folder contains the Rust code for AD-ESS.
Interesting files:

- `main.rs` provides a small example computing some metrics for AD-ESS
- `ad_ess.rs` provides a `struct AdEss` with methods for AD-ESS encoding, decoding and computing some useful metrics like average energy
- `rts.rs` provides a `struct RTS` similar to `AdEss` which uses a reversed trellis for shaping

## Installation

As AD-ESS relies on the `rug` crate which in turn uses GMP.
Compiling GMP on Windows might not work, failing the build of AD-ESS.

### From crates.io

Run `cargo add adess` to add this crate to your rust project.

## From Source

Clone this git repo.
The Rust code can be compiled and run with `cargo run`.
An optimized build can be created using `cargo build --release`.

The documentation can be compiled with `cargo doc`.

## Testing

Some tests are located in `src/tests.rs`, these can be run with `cargo test`.
