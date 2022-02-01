# ore_encoding.rs

[![Test](https://github.com/cipherstash/ore_encoding.rs/actions/workflows/test.yml/badge.svg)](https://github.com/cipherstash/ore_encoding.rs/actions/workflows/test.yml)

This is a companion package to [ore.rs](https://github.com/cipherstash/ore.rs) that can generate and manipulate `u64` plaintexts before they are encrypted by `ore.rs`. Being able to manipulate the intermediate representation of a term is critical for implementing range queries.

Currently a `OrePlaintext<u64>` can be generated from `f64`, `f32`, `u64` (no-op), `u32`, `u16`, `u8` and `bool`.

Additional functionality is provided for creating `OreRange<u64>` instances.


## TODO

- Implement support for generating `u64` plaintexts from Unix timestamps with a user-determined resolution (e.g. years, months, days, etc).

## Usage Documentation

Reference documentation is on [docs.rs/ore-encoding-rs](https://docs.rs/ore-encoding-rs).

## Build, Test and Bench

To build, run:

```
cargo build
```

To test, run:

```
cargo test
```
