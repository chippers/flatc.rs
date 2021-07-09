# Changelog

## [0.2.0+2.0.0](https://github.com/chippers/flatc.rs/commit/d04c3937751d33144687209d06f4bfa40e80fd38)

Flatbuffers `v1` will not be updating going forward, as `v2` only marks the start of a semver naming scheme and not a
significant release (although it still has breaking changes like the `v1` releases).

### Changed
  * Bumped flatbuffers to version `v2.0.0`. Note that this release isn't any more breaking than previous releases, the
    flatbuffers project is just switching to semver to be more consistent. See the [flatbuffers v2.0.0 release notes here](https://github.com/google/flatbuffers/releases/tag/v2.0.0)

## [0.2.0+1.12.0](https://github.com/chippers/flatc.rs/commit/6c749ffed6020e602a155153fbdeddd4b5811392)
### Added
  * Windows support. (untested as I cannot compile flatbuffers `1.12.0` with gcc 11.1.0)

## [0.1.1+1.12.0](https://github.com/chippers/flatc.rs/commit/d8250dfcb1f71708132820ec7fc35def0a55b3fb)
### Added
  * Small pieces of documentation.
  * `#[forbid(unsafe_code)]` (lol theres so little code)

### Fixed
  * Used a more appropriate commit for the flabuffers submodule

## [0.1.0+1.12.0](https://github.com/chippers/flatc.rs/commit/f624ad2f168d2511ecf6f5de7ce268d54fe0de63)
### Added
  * Initial release. No Windows support.