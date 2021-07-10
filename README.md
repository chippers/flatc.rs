# flatc

Builds [flatbuffers](https://github.com/google/flatbuffers) and provides the path to the built `flatc`. Intended for use
by build scripts, but can be useful otherwise if the project is using `flatc` during runtime. The flatbuffers version is
determined by the build identifier for the crate version. `v0.2.0+2.0.0` means the crate is version `0.2.0` and the
vendored flatbuffers version is `2.0.0`. There's no pressure in releasing crate version `1.0.0` since this is primarily
a build tool, so it will come eventually after nothing needs to be fixed/added for a while.

[![crates.io](https://img.shields.io/crates/v/flatc.svg)](https://crates.io/crates/flatc)
[![docs.rs](https://docs.rs/flatc/badge.svg)](https://docs.rs/flatc)
[![github actions](https://github.com/chippers/flatc.rs/workflows/ci/badge.svg)](https://github.com/chippers/flatc.rs/actions)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
