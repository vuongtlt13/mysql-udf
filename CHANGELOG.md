# Changelog

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

### Changed


## [0.1.8] - 2023-10-04

### Added

Add blake2 and blake3 hashes:

- `blake2b512`
- `blake2s256`
- `blake3`

## [0.1.7] - 2023-09-11

### Added

- Add `xxhash` hashing algorithm. This includes the following:
  - `xxhash3`
  - `xxhash32`
  - `xxhash64`
  - `xxhash` (an alias for `xxhash64`)

## [0.1.6] - 2023-04-28

### Added

- Add `uuid_to_bin` and `uuid_from_bin`/`bin_to_uuid` functions


## [0.1.5] - 2023-03-23

### Changed

- Update dependencies
- Update CI and docker files to use sparse registry, for faster build times


## [0.1.4] - 2023-01-03

Changed licensing from 'Apache-2.0' to 'Apache-2.0 OR GPL-2.0-or-later'


## [0.1.3] - 2022-12-21

### Added

- Added support for v6 and v7 UUIDs

### Changed

- Correct `uuid_ns_url` output
- (workflow) correct integration testing workflow


## [0.1.2] - 2022-12-20

### Changed

- Removed unneeded debug statement from uuid generate


## [0.1.1] - 2022-12-20

### Changed

- Corrected release notes shipped with binaries


## [0.1.0] - 2022-12-20

### Added

- Added initial functions to generate v1 and v4 UUIDs, and generate namespaces
- Added initial `lipsum` function
- Added initial `jsonify` function

<!-- next-url -->

[Unreleased]: https://github.com/pluots/udf-suite/compare/v0.1.8...HEAD
[0.1.8]: https://github.com/pluots/udf-suite/compare/v0.1.7...v0.1.8
[0.1.7]: https://github.com/pluots/udf-suite/compare/v0.1.6...v0.1.7
[0.1.6]: https://github.com/pluots/udf-suite/compare/v0.1.5...v0.1.6
[0.1.5]: https://github.com/pluots/udf-suite/compare/v0.1.4...v0.1.5
[0.1.4]: https://github.com/pluots/udf-suite/compare/v0.1.3...v0.1.4
[0.1.3]: https://github.com/pluots/udf-suite/compare/v0.1.2...v0.1.3
[0.1.2]: https://github.com/pluots/udf-suite/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/pluots/udf-suite/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/pluots/udf-suite/releases/tag/v0.1.0
