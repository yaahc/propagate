# Propagate

Error propagation tracking in Rust.

## Documentation

Clone the repo and build docs using cargo:

```
cargo doc --open
```

## Building

Propagate requires [`#[feature(try_trait_v2)]`][try] and [`#[feature(control_flow_enum)]`][control]. Build with Rust nightly:

```
cargo +nightly build
```

[try]: https://github.com/rust-lang/rust/issues/84277
[control]: https://github.com/rust-lang/rust/issues/75744