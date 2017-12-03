# Lust [![Build Status](https://travis-ci.org/SeanPrashad/Lust.svg?branch=master)](https://travis-ci.org/SeanPrashad/Lust)

A Rust library designed to provide simple data for a given file path!

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Usage

1. `get_file_name(file_path: &str)` - returns the file name (as an `OsString`), given an absolute or relative path

  ```Rust
  let file_name = get_file_name("C:/Users/JohnDoe/Desktop/repos/Lust/README.md");
  println!("{:?}", file_name);  //prints "README.md"
  ```

2. `get_file_size(file_path: &str)` - returns the file size (as a `u64`), given an absolute or relative path

  ```Rust
  let file_size = get_file_size("C:/Users/JohnDoe/Desktop/repos/Lust/README.md");
  println!("{:?}", file_size);  //prints "82"
  ```

3. `generate_sha1(file_path: &str)` - returns the sha1 hash (as a `String`), given an absolute or relative path

  ```Rust
  let sha1_hash = generate_sha1("C:/Users/JohnDoe/Desktop/repos/Lust/README.md");
  println!("{:?}", sha1_hash);  //prints "e2ae20d9ae7fcacb605c03c198e0a1c51d446f50"
  ```

4. `generate_md5(file_path: &str)` - returns the md5 hash (as a `String`), given an absolute or relative path

  ```Rust
  let md5_hash = generate_md5("C:/Users/JohnDoe/Desktop/repos/Lust/README.md");
  println!("{:?}", md5_hash);  //prints "b958ee170050ed7a2f93509f13bf16c3"
  ```

### Installing Prerequisites

Below you'll find what's required to build and compile Lust:

#### Rust and Cargo

OSX/Unix installation (via [Terminal](https://en.wikipedia.org/wiki/Terminal_(macOS))):

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Instructions for Microsoft Windows may be found **[here](https://github.com/rust-lang/cargo#compiling-from-source)**.

Further installation instructions can be found **[here](http://doc.crates.io/#installing)**.

### Building Lust

After installing the prerequisites listed above, you're ready to build!

1. First, clone the repository to your local machine via git:

  ```
  git clone https://github.com/SeanPrashad/Lust.git
  ```

  Or by downloading the `.zip` equivalent, found **[here](https://github.com/SeanPrashad/Lust/archive/master.zip)**.

  *Note*: Remember to extract the `.zip` file to a location where you conduct your work!

2. Next, `cd` into the downloaded repository using your Terminal (or Git Bash if on Windows):

  ```
  cd whereMyReposAreStored/Lust/
  ```

3. To build the source code, use:

  ```
  cargo build
  ```

*Note*: Supply the `--verbose` argument to get a more detailed output (ie. `cargo build --verbose`)

## Running the Test Suite

The test suite can be found within `src/lib.rs` in the module `tests`. To build and run all test suites, simply use:

```
cargo test
```

*Note*: Supply the `--verbose` argument to get a more detailed output (ie. `cargo test --verbose`)

## Linting

Rust comes with a built-in linter, Clippy, that's automatically configured to run when you build Lust.

To invoke Clippy on-demand, simply run:

```
cargo clippy
```

*Note*: Supply the `--verbose` argument to get a more detailed output (ie. `cargo clippy --verbose`). More information can be found **[here](https://github.com/rust-lang-nursery/rust-clippy#usage)**.


## Technologies & Tools

* [Rust](https://www.rust-lang.org/en-US/index.html) - A systems programming language
* [Crates.io](https://crates.io/) - Rust's Package Manager
* [Clippy](https://github.com/rust-lang-nursery/rust-clippy#rust-clippy) - Linting for Rust
* [Rust-Crypto](https://crates.io/crates/rust-crypto) - A library of common cryptographic algorithms

## Contributing

Any and all contributions are welcome, regardless of your programming expertise. Please refer to [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get started.

## Authors

* **Sean Prashad** - *Initial and on-going maintenance* - [@SeanPrashad](https://github.com/SeanPrashad)
* **David Humphrey** - *Documentation improvements/development suggestions* - [@humphd](https://github.com/humphd)
* **Marco Beltempo** - *Source code refinement* - [@marcobeltempo](https://github.com/marcobeltempo)

See also the list of [contributors](https://github.com/SeanPrashad/Lust/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

* #rust-beginners on Mozilla's IRC server for all of the help
* Professor David Humphrey for guiding, challenging and rewarding us with the world of Open Source
