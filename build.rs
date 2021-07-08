fn main() {
    let mut config = cmake::Config::new("flatbuffers");

    let target = std::env::var("TARGET").unwrap();
    let host = std::env::var("HOST").unwrap();
    if target.contains("msvc") && host.contains("windows") {
        config.cxxflag("/EHsc");
    }

    config.build();
}
