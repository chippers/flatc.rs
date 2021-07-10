use cmake::Config;
use std::env::var;

fn main() {
    let mut config = Config::new("flatbuffers");

    // warning C4530: C++ exception handler used, but unwind semantics are not enabled. Specify /EHsc
    // todo: see why flatbuffers doesn't seem to need to enable this in their own CMakeLists.txt
    let target = var("TARGET").unwrap();
    let host = var("HOST").unwrap();
    if target.contains("msvc") && host.contains("windows") {
        config.cxxflag("/EHsc");
    }

    config.build();
}
