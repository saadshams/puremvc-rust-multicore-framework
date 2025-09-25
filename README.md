## [PureMVC](https://puremvc.org) Rust Multicore Framework [![Rust](https://github.com/saadshams/puremvc-rust-multicore-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/PureMVC/puremvc-rust-multicore-framework/actions/workflows/rust.yml)

PureMVC is a lightweight framework for creating applications based upon the classic [Model-View-Controller](http://en.wikipedia.org/wiki/Model-view-controller) design meta-pattern. It supports [modular programming](http://en.wikipedia.org/wiki/Modular_programming) through the use of [Multiton](http://en.wikipedia.org/wiki/Multiton) Core actors instead of the [Singletons](http://en.wikipedia.org/wiki/Singleton_pattern).
* [API Docs]()

## Installation

In your Cargo.toml add:
```
[dependencies]
puremvc = "1.0.0"
```

### Development
```
cargo fetch
```

#### Build
```
cargo build           # Debug build
cargo build --release # Optimized release build
```

#### Run
```
cargo run              # Run in debug mode
cargo run --release    # Run in release mode
```

#### Test
```
cargo test
```

#### Benchmarks
```
cargo bench
```

#### Documentation
```
cargo doc --open
cargo doc --open --document-private-items  # Doc with private methods
```

#### Clean
```
cargo clean
cargo check
```

#### Publish
```
cargo publish         # Publish to crates.io
```
