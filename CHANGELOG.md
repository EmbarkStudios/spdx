# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
### Changed
- Updated to version 3.11 of the SPDX license list

## [0.3.5] - 2021-02-12
### Fixed
- Update smallvec to fix an [advisory](https://rustsec.org/advisories/RUSTSEC-2021-0003)

## [0.3.4] - 2020-03-04
### Added
- Added `Expression::iter()` which iterates over both the license requirements and the operators.

## [0.3.3] - 2020-02-29
### Changed
- Updated to version 3.8 of the SPDX license list

## [0.3.2] - 2020-01-29
### Changed
- [PR#19](https://github.com/EmbarkStudios/spdx/pull/19) added the `#[non_exhaustive]` attribute to the new `ParseMode` enum, which bumped the minimum required Rust version to use this crate to 1.40. [PR#21](https://github.com/EmbarkStudios/spdx/pull/21) removed this attribute as that enum is primarily an input for this crate, and had little benefit.

## [0.3.1] - 2020-01-28 (yanked)
### Added
- [PR#19](https://github.com/EmbarkStudios/spdx/pull/19) Added `ParseMode` enum, which has a `Lax` variant that allows certain invvalid license identifiers found in some crates on crates.io, as well as the invalid `/` expression separator. Thanks [@kornel](https://github.com/kornelski)!

## [0.3.0] - 2019-12-14
### Added
- Added `LicenseId::is_gnu` to indicate the GNU licenses (GPL, AGPL, LGPL, GFDL), which use a different suffix format than all other licenses
- `std::error::Error` is now (properly) implemented for `error::ParseError`
- `LicenseReq` not implements `From<LicenseId>`

### Changed
- `Lexer` and `Token` can now be reached via the `lexer` module
- `parser` and `expression` are no longer part of the public API
- `IS_COPYLEFT`, `IS_DEPRECATED`, `IS_FSF_LIBRE`, and `IS_OSI_APPROVED` are no longer part of the public API
- The `GFDL*` licenses are now marked as copyleft

### Fixed
- When creating a `LicenseReq` from a GNU license, the license identifier is converted into its base form,
eg. `GPL-2.0-or-later` becomes `GPL-2.0+` so that the GNU style license identifiers can be used just the same
as all of the other ones. See [this issue](https://github.com/EmbarkStudios/cargo-deny/issues/55)

## [0.2.4] - 2019-11-25
### Added
- Impl Display for Licensee

## [0.2.3] - 2019-11-07
### Changed
- Updated the list of licenses to version 3.7 of the SPDX license list

## [0.2.2] - 2019-11-06
### Added
- Added `LicenseId::full_name` which is the full name for the license, eg "MIT License" for the "MIT" short identifier

## [0.2.1] - 2019-10-21
### Added
- [#9](https://github.com/EmbarkStudios/spdx/pull/9) Added a flag for determining if a license is considered [copyleft](https://en.wikipedia.org/wiki/Copyleft). Thanks [@kain88-de](https://github.com/kain88-de)!

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

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/spdx/compare/0.3.5...HEAD
[0.3.5]: https://github.com/EmbarkStudios/spdx/compare/0.3.4...0.3.5
[0.3.4]: https://github.com/EmbarkStudios/spdx/compare/0.3.3...0.3.4
[0.3.3]: https://github.com/EmbarkStudios/spdx/compare/0.3.2...0.3.3
[0.3.2]: https://github.com/EmbarkStudios/spdx/compare/0.3.1...0.3.2
[0.3.1]: https://github.com/EmbarkStudios/spdx/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/EmbarkStudios/spdx/compare/0.2.4...0.3.0
[0.2.4]: https://github.com/EmbarkStudios/spdx/compare/0.2.3...0.2.4
[0.2.3]: https://github.com/EmbarkStudios/spdx/compare/0.2.2...0.2.3
[0.2.2]: https://github.com/EmbarkStudios/spdx/compare/0.2.1...0.2.2
[0.2.1]: https://github.com/EmbarkStudios/spdx/compare/0.2.0...0.2.1
[0.2.0]: https://github.com/EmbarkStudios/spdx/compare/0.1.0...0.2.0
[0.1.0]: https://github.com/EmbarkStudios/spdx/releases/tag/0.1.0
