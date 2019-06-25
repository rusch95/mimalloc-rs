A Rust allocator backed by mimalloc
===

[![Travis-CI Status]][travis]

This crates provides a Rust `#[global_allocator]` backed by [`mimalloc`].

See also the [`mimalloc-sys`] crate providing raw FFI bindings to [`mimalloc`].

## Design Decisions

mimalloc-rs aims to allow for a function-by-function port of mimalloc, https://github.com/microsoft/mimalloc (written in C), to Rust. It does so by dynamically linking the Rust code to mimalloc, so that functions can be replaced one-by-one with a suite of tests run in between. Eventually, once all of the code is ported over to Rust, the FFI and porting infrastructure will be replaced with direct calls to the rust_impl code.

c_impl refers to the forked mimalloc code written in C 

rust_impl refers to the mimalloc code ported over to Rust.

## Testing

```
# Populate the submodule fork of mimalloc
$ git submodule init
$ git submodule update

# Build the rust impl library
$ cd mimalloc-rs/mimalloc-sys/rustimpl
$ cargo build

# Run tests
$ cd mimalloc-rs
$ LD_LIBRARY_PATH=mimalloc-sys/rustimpl/target/debug/ cargo test
```

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `mimalloc-sys` by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.

[`mimalloc-sys`]: https://crates.io/crates/mimalloc-sys
[`mimalloc`]: https://github.com/microsoft/mimalloc
[travis]: https://travis-ci.com/gnzlbg/mimallocator
[Travis-CI Status]: https://travis-ci.com/gnzlbg/mimallocator.svg?branch=master
