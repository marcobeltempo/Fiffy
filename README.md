# Fiffy [![Build Status](https://travis-ci.org/SeanPrashad/Fiffy.svg?branch=master)](https://travis-ci.org/SeanPrashad/Fiffy) [![Build Status]( https://img.shields.io/crates/v/fiffy.svg)](https://crates.io/crates/fiffy)

A Rust library designed to provide simple data for a given file path!

## Table of Contents

  * [Installing Prerequisites](#installing-prerequisites)
  * [Building Fiffy](#building-fiffy)
  * [Running the Test Suite](#running-the-test-suite)
  * [Linting](#linting)
  * [Usage](#usage)
  * [Tools and Technologies](#tools-and-technologies)
  * [Contributing](#contributing)
  * [Authors](#authors)
  * [License](#license)
  * [Acknowledgments](#acknowledgments)

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Installing Prerequisites

Below you'll find what's required to build and compile Fiffy:

#### Rust and Cargo

OSX/Unix installation (via [Terminal](https://en.wikipedia.org/wiki/Terminal_(macOS))):

```
curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

Instructions for Microsoft Windows may be found **[here](https://github.com/rust-lang/cargo#compiling-from-source)**.

Additional installation instructions can be found **[here](http://doc.crates.io/#installing)**.

### Building Fiffy

After installing the prerequisites listed above, you're ready to build!

1. First, clone the repository to your local machine via git:

  ```
  git clone https://github.com/SeanPrashad/Fiffy.git
  ```

  Or by downloading the `.zip` equivalent, found **[here](https://github.com/SeanPrashad/Fiffy/archive/master.zip)**.

  *Note*: Remember to extract the `.zip` file to a location where you conduct your work!

2. Next, `cd` into the downloaded repository using your Terminal (or Git Bash if on Windows):

  ```
  cd whereMyReposAreStored/Fiffy/
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

Rust comes with a built-in linter, Clippy, that's automatically configured to run when you build Fiffy.

To invoke Clippy on-demand, simply run:

```
cargo clippy
```

*Note*: Supply the `--verbose` argument to get a more detailed output (ie. `cargo clippy --verbose`). More information can be found **[here](https://github.com/rust-lang-nursery/rust-clippy#usage)**.

## Usage

1. `get_file_name(file_path: &str)` - returns the file name (as an [OsString](https://doc.rust-lang.org/std/ffi/struct.OsString.html)), given an absolute or relative path

  ```Rust
  let file_path = "/home/kim/mydata.txt";
  let file_name = get_file_name(file_path);
  println!("{:?}", file_name);  //prints "mydata.txt"
  ```

2. `get_file_size(file_path: &str)` - returns the file size (as a [u64](http://manishearth.github.io/rust-internals-docs/std/primitive.u64.html)), given an absolute or relative path

  ```Rust
  let file_path = "/home/kim/mydata.txt";
  let file_size = get_file_size(file_path);
  println!("{:?}", file_size);  //prints "129" - (e.g., the file is 129 bytes on disk)
  ```

3. `generate_sha1(file_path: &str)` - returns the sha1 digest (as a [String](https://doc.rust-lang.org/std/string/struct.String.html)), given an absolute or relative path

  ```Rust
  let text = "The quick brown fox jumps over the lazy dog"
  let sha1_digest = generate_sha1(text);
  println!("{:?}", sha1_digest);  //prints "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12"
  ```

4. `generate_md5(file_path: &str)` - returns the md5 digest (as a [String](https://doc.rust-lang.org/std/string/struct.String.html)), given an absolute or relative path

  ```Rust
  let text = "The quick brown fox jumps over the lazy dog"
  let md5_digest = generate_md5(text);
  println!("{:?}", md5_digest);  //prints "9e107d9d372bb6826bd81d3542a419d6"
  ```

## Tools and Technologies

* [Rust](https://www.rust-lang.org/en-US/index.html) - A systems programming language
* [Crates.io](https://crates.io/) - Rust's Package Manager
* [Clippy](https://github.com/rust-lang-nursery/rust-clippy#rust-clippy) - Linting for Rust
* [Rust-Crypto](https://crates.io/crates/rust-crypto) - A library of common cryptographic algorithms

## Contributing

Any and all contributions are welcome, regardless of your programming expertise. Please refer to [CONTRIBUTING.md](CONTRIBUTING.md) for details on how to get started.

## Authors

* **Sean Prashad** - *Initial and on-going maintenance* - [@SeanPrashad](https://github.com/SeanPrashad)
* **David Humphrey** - *Initial documentation and development improvements/suggestions* - [@humphd](https://github.com/humphd)
* **Marco Beltempo** - *Source code contributor* - [@marcobeltempo](https://github.com/marcobeltempo)
* **Dan Epstein** - *Documentation contributor* - [@Securter](https://github.com/Securter)

See also the list of [contributors](https://github.com/SeanPrashad/Fiffy/graphs/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

## Acknowledgments

* #rust-beginners on Mozilla's IRC server for all of the help
* Professor David Humphrey for guiding, challenging and rewarding us with the world of Open Source
