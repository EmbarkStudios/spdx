# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2019-10-03
### Added
- Added a `Expression` which can parse and validate an SPDX license expression is
both syntactically and semantically correct, as well as evaluate the expression via
a user provided callback
- Added an update exe for pulling new SDPX information, copied from https://github.com/rust-lang-nursery/license-exprs
- Added support for some of the metadata available from the SPDX format, namely "IsDeprecated", "IsFSFLibre", and "IsOSIApproved"

### Changed
- Uhm...everything. I hope no one was using 0.1.0.
- Use a better lexer, mostly copied from https://github.com/rust-lang-nursery/license-exprs/pull/29

## [0.1.0] - 2019-09-02
### Added
- Initial add of spdx crate, based primarly on [`license-exprs`](https://github.com/rust-lang-nursery/license-exprs)

[Unreleased]: https://github.com/EmbarkStudios/spdx/compare/0.2.0...HEAD
[0.2.0]: https://github.com/EmbarkStudios/cargo-deny/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/spdx/releases/tag/0.1.0
