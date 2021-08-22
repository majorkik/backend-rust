# Rust backend

[![Status](https://img.shields.io/badge/Status-in%20progress-success)]()
[![Rust & Cargo](https://img.shields.io/badge/Rust%20%26%20Cargo-1.54.0-blueviolet)](https://www.rust-lang.org/)
[![Rocket](https://img.shields.io/badge/Rocket-0.5.0--rc.1-blue)](https://rocket.rs/)
[![Diesel](https://img.shields.io/badge/Diesel-1.4.7-yellow)](https://diesel.rs/)

## Getting started  

### Run & Build

```shell
cargo build # Update dependencies and build project
cargo run   # Update dependencies, build and run the project
```

### Testing

```shell
cargo test  # Running tests
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

## License
```
MIT License

Copyright (c) 2021 Rodion Belovitskiy

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction, including
without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to
the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT
LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN
NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF  TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
```

[rustfmt]: https://github.com/rust-lang/rustfmt "Rustfmt - tool for formatting Rust code"
