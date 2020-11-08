# readstat-rs
Example Rust binary that counts the number of records in a SAS binary file (`sas7bdat`).

## ReadStat
The Rust binary is only possible due to the excellent [ReadStat](https://github.com/WizardMac/ReadStat) library developed by [Evan Miller](https://www.evanmiller.org).

The [ReadStat](https://github.com/WizardMac/ReadStat) repository is included as a [git submodule](https://git-scm.com/book/en/v2/Git-Tools-Submodules) within this repository.  In order to build and link, first a [readstat-sys](https://github.com/curtisalexander/readstat-rs/readstat-sys) crate is created.  Then the [readstat](https://github.com/curtisalexander/readstat-rs/readstat) binary utilizes `readstat-sys` as a dependency.

The binary binds to `ReadStat` and follows the [reading files](https://github.com/WizardMac/ReadStat#library-usage-reading-files) example in the `ReadStat` repository.

## Run
The binary is run from within the `readstat` directory using the following.  This assumes Rust and cargo have been [installed and setup](https://rustup.rs/).

```sh
cd readstat
cargo run -- --sas /some/dir/to/example.sas7bdat
```

The record count (along with other, extraneous program information) is written to standard out.

## Testing
To run unit / integration tests, run the following within the `readstat` directory.

```
cargo test
```

To ensure no memory leaks, run [valgrind](https://valgrind.org/).

```
valgrind ./target/debug/deps/get_row_count_test-11793a929ad2468f
```

## Goals

### Short Term
Short term the developed binary was a helpful exercise in binding to a C library using [bindgen](https://rust-lang.github.io/rust-bindgen/) and the [Rust FFI](https://doc.rust-lang.org/nomicon/ffi.html).  It definitely required a review of C pointers (and for which I claim no expertise)!

### Long Term
Uncertain of the long term goals of this repository.  Possibilities include:
- Building a Rust library that works with `sas7bdat` files
- Developing a command line tool that expands the functionality made available by the [readstat](https://github.com/WizardMac/ReadStat#command-line-usage) command line tool
- Develop a command line tool that performs transformations from `sas7bdat` to other file types
    - `csv`
    - `json`
    - `parquet`
    - `arrow`

## Resources
The following have been **_incredibly_** helpful in developing.
- [How to not RiiR](http://adventures.michaelfbryan.com/posts/how-not-to-riir/#building-chmlib-sys)
- [Making a *-sys crate](https://kornel.ski/rust-sys-crate)
- Rust FFI: Microsoft Flight Simulator SDK
    - [Part 1](https://youtu.be/jNNz4h3iIlw)
    - [Part 2](https://youtu.be/ugiR9M16fwg)