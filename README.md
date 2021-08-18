# Rust backend

[![Status](https://img.shields.io/badge/Status-in%20progress-success)]()
[![Rust & Cargo](https://img.shields.io/badge/Rust%20%26%20Cargo-1.54.0-blueviolet)](https://www.rust-lang.org/)
[![Rocket](https://img.shields.io/badge/Rocket-0.5.0--rc.1-blue)](https://rocket.rs/)
[![Diesel](https://img.shields.io/badge/Diesel-1.4.7-yellow)](https://diesel.rs/)

## Getting started  

```shell
cargo build # Update dependencies and build project
cargo run   # Update dependencies, build and run the project
```

### Code style

The project follows the official coding style.
For automatic formatting, a tool is used - [**Rustftm**][rustfmt].

> :information_source: [Rustfmt][rustfmt] - tool for formatting Rust code according to style guidelines.

```shell
# Installation guide 

# Install tool
rustup component add rustfmt

# Run Rustfmt
cargo fmt
```

[rustfmt]: https://github.com/rust-lang/rustfmt "Rustfmt - tool for formatting Rust code"
