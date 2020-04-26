fn main() {
    let mut config = cmake::Config::new("flatbuffers");

    // specially set the visual studio generator version on msvc windows
    // this generator is recommended by https://google.github.io/flatbuffers/flatbuffers_guide_building.html
    /*    let target = std::env::var("TARGET").unwrap();
    let host = std::env::var("HOST").unwrap();
    if target.contains("msvc") && host.contains("windows") {
        config.generator("Visual Studio 15 2017");
    }*/

    config.build();
}
