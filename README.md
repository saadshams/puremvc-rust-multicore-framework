## [PureMVC](https://puremvc.org) Rust Multicore Framework [![Rust](https://github.com/saadshams/puremvc-rust-multicore-framework/actions/workflows/rust.yml/badge.svg)](https://github.com/PureMVC/puremvc-rust-multicore-framework/actions/workflows/rust.yml)

PureMVC is a lightweight framework for creating applications based upon the classic [Model-View-Controller](http://en.wikipedia.org/wiki/Model-view-controller) design meta-pattern. It supports [modular programming](http://en.wikipedia.org/wiki/Modular_programming) through the use of [Multiton](http://en.wikipedia.org/wiki/Multiton) Core actors instead of the [Singletons](http://en.wikipedia.org/wiki/Singleton_pattern).
* [API Docs]()

## Installation

Clone this repo from GitHub:
```
git clone https://github.com/saadshams/puremvc-rust-multicore-framework.git
```

In your terminal, locate the project folder:
```
cd puremvc-rust-multicore-framework
```

In your Cargo.toml add:
```
[dependencies]
ruex = "0.1.5"

[lib]
name = "puremvc"
path = "src/lib.rs"

[[bin]]
name = "puremvc"
path = "src/main.rs"
```

In the src/ dir, create a main.rs file and paste the code below;
```
use puremvc::patterns::Facade;

fn main() {
    // Create a factory closure that returns a new Facade
    let _facade_factory = |key: &str| -> Facade {
        Facade::new(key)
    };
}
```

In src/patterns/mod.rs file, Add pub to the facade module
```
pub mod facade
```

## Dependencies
```
cargo fetch
```

## Build
```
cargo build           # Debug build
cargo build --release # Optimized release build
```

## Run
```
cargo run              # Run in debug mode
cargo run --release    # Run in release mode
```

## Test
```
cargo test
```

## Benchmarks
```
cargo bench
```

## Documentation
```
cargo doc --open
```

## Clean
```
cargo clean
cargo check
```

# Publish
```
cargo publish         # Publish to crates.io
```
