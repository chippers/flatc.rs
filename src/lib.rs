//! Builds [flatbuffers] and provides the path to the built `flatc` in the
//! public API. Typically meant to be used by build scripts, but can be useful
//! otherwise if the project itself is using `flatc` during runtime.
//!
//! [flatbuffers]: https://github.com/google/flatbuffers

#[forbid(unsafe_code)]

/// Path of the built `flatc` executable.
pub fn flatc() -> &'static std::path::Path {
    std::path::Path::new(concat!(env!("OUT_DIR"), "/bin/flatc"))
}

#[test]
fn flatc_exists() {
    assert!(flatc().exists())
}
