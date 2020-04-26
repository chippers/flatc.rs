# flatc

Builds [flatbuffers](https://github.com/google/flatbuffers) and provides the
path to the built `flatc` in the public API. Typically meant to be used by
build scripts, but can be useful otherwise if the project itself is using
`flatc` during runtime. Version `1.0` will not be released until windows is
fully supported.

[![crates.io](https://meritbadge.herokuapp.com/flatc)](https://crates.io/crates/flatc)
[![docs.rs](https://docs.rs/flatc/badge.svg)](https://docs.rs/flatc)
[![github actions](https://github.com/chippers/flatc.rs/workflows/ci/badge.svg)](https://github.com/chippers/flatc.rs/actions)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Windows support

Currently, windows is not supported under either MSVC or GNU. I don't have
access to windows development environments so I am not able to debug compiling
on windows except through CI, which is rather unproductive. There is an
[open ticket] for windows support, which will lead to a `1.0` release.

[open ticket]: https://github.com/chippers/flatc.rs/issues/1

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.