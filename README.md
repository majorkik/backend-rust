<h1 align="center">Rust web application</h1>

<p align="center">
    <a>
      <img src="https://img.shields.io/badge/Status-in%20progress-success?style=for-the-badge" alt="Status">
  </a>

  <a href="https://www.rust-lang.org/">
      <img src="https://img.shields.io/badge/Rust%20%26%20Cargo-1.54.0-blueviolet?style=for-the-badge" alt="Rust & Cargo">
  </a>

  <a href="https://rocket.rs/">
      <img src="https://img.shields.io/badge/Rocket-0.5.0--rc.1-blue?style=for-the-badge" alt="Rocket">
  </a>

  <a href="https://diesel.rs/">
      <img src="https://img.shields.io/badge/Diesel-1.4.7-yellow?style=for-the-badge" alt="Diesel">
  </a>

  <a href="https://www.postgresql.org/">
      <img src="https://img.shields.io/badge/Database-PostgreSQL-blueviolet?style=for-the-badge" alt="Database">
  </a>
</p>

## :dart: Tech-stack

- [**100% Rust**][rust]
- **Network:** [Rocket][github-rocket]
- **ORM:** [Diesel][github-diesel]
- **Configuration:** [Dotenv][github-dotenv]
- **Database:** PostgreSQL

## Getting started

### Configure environment variables

Database configurations and other settings are stored in a `.env` file.

First you need to rename the file `.env.sample` to` .env`, after which you need to fill it with your data.

### Database (PostgreSQL)

[**Diesel (ORM)**][diesel] is used to work with the database.

> :information_source: More information on setting up **Diesel CLI** can be found on the [official website under Getting Started][diesel-getting-started].

#### Installing Diesel CLI

```shell
# Installing diesel_cli
$ cargo install diesel_cli --no-default-features --features postgres
```

#### Setup diesel cli

First, you need to rename the `.env.sample` file to `.env` and fill in all the parameters with your data, then run the
command `diesel setup`.

```shell
# Will create a database (if not already created) and create an empty folder for migrations. 
$ diesel setup 
```

### :rocket: Run & Build

```shell
$ cargo build # Update dependencies and build project
$ cargo run   # Update dependencies, build and run the project
```

## Testing

```shell
$ cargo test  # Running tests
```

## Code style

The project follows the official coding style. For automatic formatting, a tool is used - [**Rustftm**][rustfmt].

> :information_source: [Rustfmt][rustfmt] - tool for formatting Rust code according to style guidelines.

```shell
# Installation guide 

# Install tool
$ rustup component add rustfmt

# Run Rustfmt
$ cargo fmt
```

# Bazel

```shell
# Use bazel or bazelisk
# Step 1: Make sure Cargo.lock exists, otherwise generate it
cargo build

# Step 2: Make sure Cargo.Bezel.lock exists, otherwise create it
touch Cargo.Bazel.lock

# Step 3: Regenerate the dependencies represented by the rule
CARGO_BAZEL_REPIN=true bazelisk sync --only=crate_index  

# Step 4: Build or Run
bazelisk build :app
# or
bazelisk run :app 

# (For windows build)
bazelisk --windows_enable_symlinks run --enable_runfiles //:app --@rules_rust//rust/toolchain/channel=nightly
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

[rust]: https://www.rust-lang.org/ "Rust"

[rustfmt]: https://github.com/rust-lang/rustfmt "Rustfmt - tool for formatting Rust code"

[diesel-getting-started]: https://diesel.rs/guides/getting-started "Diesel - Getting Started"

[diesel]: https://diesel.rs/ "A safe, extensible ORM and Query Builder for Rust"

[github-diesel]: https://github.com/diesel-rs/diesel "Github: Diesel"

[github-rocket]: https://github.com/SergioBenitez/Rocket "Github: Rocket"

[github-dotenv]: https://github.com/dotenv-rs/dotenv "Github: Dotenv"