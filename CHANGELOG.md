# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.1](https://github.com/vakamo-labs/openfga-client/compare/v0.5.0...v0.5.1) - 2026-01-05

### Added

- Add ModelManager retry logic after model write ([#30](https://github.com/vakamo-labs/openfga-client/pull/30))

## [0.5.0](https://github.com/vakamo-labs/openfga-client/compare/v0.4.0...v0.5.0) - 2025-12-30

### Added

- tonic 14, openfga on_duplicate / on_missing write options ([#28](https://github.com/vakamo-labs/openfga-client/pull/28))
- add TLS support for HTTPS endpoints ([#25](https://github.com/vakamo-labs/openfga-client/pull/25))

### Other

- Bump MSRV to 1.88 ([#26](https://github.com/vakamo-labs/openfga-client/pull/26))

## [0.4.0](https://github.com/vakamo-labs/openfga-client/compare/v0.3.0...v0.4.0) - 2025-09-23

### Added

- [**breaking**] pass more data into `MigrationFn` ([#16](https://github.com/vakamo-labs/openfga-client/pull/16))

### Fixed

- Remove protoc-gen-prost submodule
- [**breaking**] `read_all_pages` now correctly reads all tuples without filters ([#19](https://github.com/vakamo-labs/openfga-client/pull/19))

### Other

- *(deps)* update prost-types requirement from 0.13 to 0.14 ([#13](https://github.com/vakamo-labs/openfga-client/pull/13))
- *(deps)* bump actions/checkout from 4 to 5 ([#18](https://github.com/vakamo-labs/openfga-client/pull/18))

## [0.3.0](https://github.com/vakamo-labs/openfga-client/compare/v0.2.0...v0.3.0) - 2025-06-27

### Added

- [**breaking**] strip `Option` in `batch_check` due to prost's handling of `oneof` ([#15](https://github.com/vakamo-labs/openfga-client/pull/15))

### Other

- increase visibility of DEVELOPMENT.md ([#14](https://github.com/vakamo-labs/openfga-client/pull/14))
- Add test get multiple stores ([#10](https://github.com/vakamo-labs/openfga-client/pull/10))

## [0.2.0](https://github.com/vakamo-labs/openfga-client/compare/v0.1.2...v0.2.0) - 2025-03-03

### Added

- Migration manager, extended client ([#5](https://github.com/vakamo-labs/openfga-client/pull/5))

### Other

- *(docs)* Fix typo
- *(docs)* Include SDK

## [0.1.2](https://github.com/vakamo-labs/openfga-client/compare/v0.1.1...v0.1.2) - 2025-02-27

### Fixed

- Update Middle to prevent Client Credentials not beeing updated correctly (#3)

## [0.1.1](https://github.com/vakamo-labs/openfga-client/compare/v0.1.0...v0.1.1) - 2025-02-25

### Fixed

- Remove protoc dependency for wkt types, fix CI (#1)
