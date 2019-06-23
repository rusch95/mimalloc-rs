A Rust allocator backed by mimalloc
===

[![Travis-CI Status]][travis]

This crates provides a Rust `#[global_allocator]` backed by [`mimalloc`].

See also the [`mimalloc-sys`] crate providing raw FFI bindings to [`mimalloc`].

## Testing

```
# Populate the submodule fork of mimalloc
$ git submodule init
$ git submodule update
# Run tests
$ cargo test
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
