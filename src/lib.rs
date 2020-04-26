pub fn flatc() -> &'static std::path::Path {
    std::path::Path::new(concat!(env!("OUT_DIR"), "/bin/flatc"))
}

#[test]
fn flatc_exists() {
    let p = flatc();
    println!("{:#?}", p);
    assert!(p.exists())
}
