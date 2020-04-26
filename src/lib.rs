#[forbid(unsafe_code)]

/// Path of the built `flatc` executable.
pub fn flatc() -> &'static std::path::Path {
    std::path::Path::new(concat!(env!("OUT_DIR"), "/bin/flatc"))
}

#[test]
fn flatc_exists() {
    assert!(flatc().exists())
}
