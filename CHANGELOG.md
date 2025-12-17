<!-- markdownlint-disable blanks-around-headings blanks-around-lists no-duplicate-heading -->

# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate
## [0.13.3] - 2025-12-17
### Changed
- [PR#90](https://github.com/EmbarkStudios/spdx/pull/90) changed this crate to default to `no_std`, unless the `detection` feature is enabled, closing [PR#89](https://github.com/EmbarkStudios/spdx/pull/89). Thanks [@so-rose](https://github.com/so-rose)!
- [PR#91](https://github.com/EmbarkStudios/spdx/pull/91) changed the license terms from `MIT OR Apache2.0` to just `Apache-2.0`, as [PR#84](https://github.com/EmbarkStudios/spdx/pull/84) merged external code that was only licensed under `Apache-2.0`. See [#88](https://github.com/EmbarkStudios/spdx/pull/88) for the original raising of this mismatch.

## [0.13.2] - 2025-12-04
### Fixed
- [PR#87](https://github.com/EmbarkStudios/spdx/pull/87) fixed a code generation bug introduced in [PR#86](https://github.com/EmbarkStudios/spdx/pull/86) that caused `LicenseId::text()` to return the incorrect license text.

## [0.13.1] - 2025-12-03
### Changed
- [PR#86](https://github.com/EmbarkStudios/spdx/pull/86) changes the license detection for GNU licenses (GPL, AGPL, LGPL, GFDL). In 0.13.0 they would be detected as their deprecated (eg. `AGPL-3.0`) variant due to the variants (root, `-only`, and `-or-later`) using the exact same license text. GNU licenses are now detected as the `<root>-or-later` version. This is an arbitrary decision, which I hope to not change, but if someone makes a convincing argument it could be.

## [0.13.0] - 2025-12-02
### Added
- [PR#84](https://github.com/EmbarkStudios/spdx/pull/84) resolved [#67](https://github.com/EmbarkStudios/spdx/issues/67) by inlining the `askalono` crate to allow detection of license texts or headers from arbitrary text data. There are multiple features flags associated with this new feature.
- [PR#85](https://github.com/EmbarkStudios/spdx/pull/85) resolved [#82](https://github.com/EmbarkStudios/spdx/issues/82) by optionally allowing the parsing of unknown identifiers via `ParseMode::allow_unknown`. Unknown identifiers are either treated as `LicenseRef-<unknown identifier>` or `AdditionRef-<unknown identifier>` depending on their position. Unknown identifiers on positions that are invalid for either licenses or exceptions are still considered parse errors.

## [0.12.0] - 2025-08-19
### Added
- [PR#81](https://github.com/EmbarkStudios/spdx/pull/81) resolved [#68](https://github.com/EmbarkStudios/spdx/issues/68) by adding support for the `WITH [%s"DocumentRef-"(idstring)":"]%s"AdditionRef-"(idstring)` syntax. Thanks [@weihanglo](https://github.com/weihanglo)!

## [0.11.1] - 2025-08-11
### Changed
- [PR#80](https://github.com/EmbarkStudios/spdx/pull/80) changed how `Licensee::satisfies` works for GNU licenses again, it now requires that the license ids match exactly. This is incredibly pedantic but means it's up to consumers if the want to have a smarter comparison, I just don't want to have to care about GNU licenses, ever.

### Fixed
- [PR#80](https://github.com/EmbarkStudios/spdx/pull/80) reverted a change introduced in [PR#78] that would auto-fixup GNU licenses to their non-deprecated forms eg. `GPL-2.0` => `GPL-2.0-only`. This is no longer done, resolving [#79](https://github.com/EmbarkStudios/spdx/issues/79).

## [0.11.0] - 2025-08-08
### Changed
- [PR#78] removed `ParseMode::allow_lower_case_operators`, newer revisions of the SPDX spec allow all lower-case operators, making the option pointless.
- [PR#78] added `ParseMode::allow_deprecated`, which will cause an error if a deprecated license identifier is used, `false` in `LAX` and `true` in `STRICT`.
- [PR#78] changed the various imprecise names for GPL licenses to be mapped to the non-deprecated `-only` versions.
- [PR#78] `Expression::canonicalize` now always changes GNU licenses to be `-only` or `-or-later` as the bare identifiers are deprecated.

### Added
- [PR#78] added `LicenseId::version` to retrieve the numeric version of the license if it has one.
- [PR#78] added `LicenseId::base` to retrieve the base name of the license.
- [PR#78] added `gnu_license_id` which attempts to retrieve the license id for a GNU license from its base identifier. This retrieves the `-only` or `-or-later` license that matches.
- [PR#78] added `Licensee::parse_mode`, `Licensee::parse` now forwards to that function with `ParseMode::STRICT`.
- [PR#78] added `Reason::GnuPlusWithSuffix` and `Reason::DeprecatedLicenseId` as errors.

### Fixed
- [PR#78] fixed an issue where `Licensee::satisfies` would not properly allow some licenses if the version was not at the end when using a `+`, notably the BSD licenses have the version in the middle of the license id.
- [PR#78] fixed the handling of GNU licenses in `Licensee::satisfies`, at least to my best understanding.
    | Licensee | GPL-1.0-only  | GPL-1.0-or-later | GPL-2.0-only | GPL-2.0-or-later | GPL-3.0-only | GPL-3.0-or-later |
    | ----------------- | -- | -- | -- | -- | -- | -- |
    | GPL-1.0-only      | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
    | GPL-1.0-or-later  | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
    | GPL-2.0-only      | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
    | GPL-2.0-or-later  | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
    | GPL-3.0-only      | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
    | GPL-3.0-or-later  | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |

## [0.10.9] - 2025-07-12
### Changed
- [PR#74](https://github.com/EmbarkStudios/spdx/pull/76) update SPDX license list to 3.27.0.

## [0.10.8] - 2024-12-31
### Changed
- [PR#74](https://github.com/EmbarkStudios/spdx/pull/74) update SPDX license list to 3.26.0.

## [0.10.7] - 2024-11-15
### Changed
- [PR#72](https://github.com/EmbarkStudios/spdx/pull/72) update SPDX license list to 3.25.0.

## [0.10.6] - 2024-05-31
### Changed
- [PR#70](https://github.com/EmbarkStudios/spdx/pull/70) update SPDX license list to 3.24.0.

## [0.10.4] - 2024-02-26
### Changed
- [PR#65](https://github.com/EmbarkStudios/spdx/pull/65) update SPDX license list to 3.23.

## [0.10.3] - 2024-01-04
### Changed
- [PR#63](https://github.com/EmbarkStudios/spdx/pull/63) update SPDX license list to 3.22.

### Added
- [PR#64](https://github.com/EmbarkStudios/spdx/pull/64) resolved [#56](https://github.com/EmbarkStudios/spdx/issues/56) by adding `Expression::canonicalize` which fixes otherwise valid expressions into a form parsable with `ParseMode::STRICT`

## [0.10.2] - 2023-07-14
### Changed
- [PR#61](https://github.com/EmbarkStudios/spdx/pull/61) updated the SPDX license list from `3.20` => `3.21`.

### Fixed
- [PR#60](https://github.com/EmbarkStudios/spdx/pull/60) fixed a few typos.

## [0.10.1] - 2023-04-06
### Changed
- [PR#59](https://github.com/EmbarkStudios/spdx/pull/59) updated the SPDX license list from `3.19` => `3.20`.

## [0.10.0] - 2022-12-20
### Changed
- [PR#57](https://github.com/EmbarkStudios/spdx/pull/57) updated the SPDX license list from `3.18` => `3.19`.

## [0.9.0] - 2022-08-25
### Changed
- [PR#55](https://github.com/EmbarkStudios/spdx/pull/55) updated the SPDX license list from `3.14` => `3.18`.

## [0.8.1] - 2022-02-04
### Changed
- [PR#51](https://github.com/EmbarkStudios/spdx/pull/51) updates the crates.io metadata for the crate.

## [0.8.0] - 2021-12-21
### Changed
- [PR#50](https://github.com/EmbarkStudios/spdx/pull/50) changed `ParseMode` to be a struct with several fields to give finer grained control over which parts of expression parsing/evaluation can be relaxed. Thanks [@Turbo87](https://github.com/Turbo87)!

## [0.7.0] - 2021-11-23
### Changed
- [PR#48](https://github.com/EmbarkStudios/spdx/pull/48) resolved [#45](https://github.com/EmbarkStudios/spdx/issues/45) by making the original error string owned in the case of a parse error, simplifying the handling of errors. Thanks [@hoijui](https://github.com/hoijui)!
- [PR#49](https://github.com/EmbarkStudios/spdx/pull/49) bumped the MSRV to 1.56.1, as well as moving to the 2021 edition and setting the `rust-version` there.

## [0.6.2] - 2021-10-21
### Fixed
- [PR#44](https://github.com/EmbarkStudios/spdx/pull/44) fixed the `Display` for various GNU licenses, since they are "special" and diverge from the SPDX spec for reasons. Thanks [@mmurto](https://github.com/mmurto)!

### Added
- [PR#43](https://github.com/EmbarkStudios/spdx/pull/43) added the `text` feature flag, which includes the full license and exception texts. They can be retrieved via `LicenseId::text` and `ExceptionId::text` respectively.
- [PR#43](https://github.com/EmbarkStudios/spdx/pull/43) added the `Expression::minimized_requirements` method which allows a set of potential licensees be reduced down to the minimum possible requirements for an expression.

## [0.6.1] - 2021-10-04
### Added
- [PR#41](https://github.com/EmbarkStudios/spdx/pull/41) added a `NOASSERTION` license. Hopefully this will become part of [spec](https://github.com/spdx/spdx-spec/issues/50) in the future.

## [0.6.0] - 2021-08-16
### Changed
- [PR#40](https://github.com/EmbarkStudios/spdx/pull/40) updated the SPDX license list from `3.11` => `3.14`.

### Fixed
- [PR#40](https://github.com/EmbarkStudios/spdx/pull/40) resolved [#39](https://github.com/EmbarkStudios/spdx/issues/39) by taking the `GFDL` exceptional differences from all other licenses (include the other GNU ones) into account.

## [0.5.0] - 2021-07-20
### Changed
- [PR#38](https://github.com/EmbarkStudios/spdx/pull/38) fixed various clippy lints which also bumps the MSRV to [1.53.0](https://blog.rust-lang.org/2021/06/17/Rust-1.53.0.html). Previously, PR#37 had bumped the MSRV to 1.52 so now this crate will check the MSRV so changes are intentional.
- [PR#38](https://github.com/EmbarkStudios/spdx/pull/38) replaced the unmaintained `difference` crate with `similar-asserts`.

## [0.4.1] - 2021-06-14
### Changed
- [PR#37](https://github.com/EmbarkStudios/spdx/pull/37) removed the dependencies on regex and lazy_static used for parsing some license expression parts, which gives a nice compile speed up with no behavior changes. Thanks [@Swagadon](https://github.com/Swagadon)!

## [0.4.0] - 2021-03-26
### Changed
- Renamed `LicenseItem::SPDX` => `LicenseItem::Spdx` and `Token::SPDX` => `Token::Spdx`.

## [0.3.6] - 2021-02-12
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
- [PR#19](https://github.com/EmbarkStudios/spdx/pull/19) Added `ParseMode` enum, which has a `Lax` variant that allows certain invalid license identifiers found in some crates on crates.io, as well as the invalid `/` expression separator. Thanks [@kornel](https://github.com/kornelski)!

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
- When creating a `LicenseReq` from a GNU license, the license identifier is converted into its base form, eg. `GPL-2.0-or-later` becomes `GPL-2.0+` so that the GNU style license identifiers can be used just the same as all of the other ones. See [this issue](https://github.com/EmbarkStudios/cargo-deny/issues/55)

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
- Added an update exe for pulling new SPDX information, copied from <https://github.com/rust-lang-nursery/license-exprs>
- Added support for some of the metadata available from the SPDX format, namely "IsDeprecated", "IsFSFLibre", and "IsOSIApproved"

### Changed
- Uhm...everything. I hope no one was using 0.1.0.
- Use a better lexer, mostly copied from <https://github.com/rust-lang-nursery/license-exprs/pull/29>

## [0.1.0] - 2019-09-02
### Added
- Initial add of spdx crate, based primarily on [`license-exprs`](https://github.com/rust-lang-nursery/license-exprs)


[PR#78]: https://github.com/EmbarkStudios/spdx/pull/78

<!-- next-url -->
[Unreleased]: https://github.com/EmbarkStudios/spdx/compare/0.13.3...HEAD
[0.13.3]: https://github.com/EmbarkStudios/spdx/compare/0.13.2...0.13.3
[0.13.2]: https://github.com/EmbarkStudios/spdx/compare/0.13.1...0.13.2
[0.13.1]: https://github.com/EmbarkStudios/spdx/compare/0.13.0...0.13.1
[0.13.0]: https://github.com/EmbarkStudios/spdx/compare/0.12.0...0.13.0
[0.12.0]: https://github.com/EmbarkStudios/spdx/compare/0.11.1...0.12.0
[0.11.1]: https://github.com/EmbarkStudios/spdx/compare/0.11.0...0.11.1
[0.11.0]: https://github.com/EmbarkStudios/spdx/compare/0.10.9...0.11.0
[0.10.9]: https://github.com/EmbarkStudios/spdx/compare/0.10.9...0.10.9
[0.10.9]: https://github.com/EmbarkStudios/spdx/compare/0.10.8...0.10.9
[0.10.8]: https://github.com/EmbarkStudios/spdx/compare/0.10.7...0.10.8
[0.10.7]: https://github.com/EmbarkStudios/spdx/compare/0.10.6...0.10.7
[0.10.6]: https://github.com/EmbarkStudios/spdx/compare/0.10.5...0.10.6
[0.10.4]: https://github.com/EmbarkStudios/spdx/compare/0.10.3...0.10.4
[0.10.3]: https://github.com/EmbarkStudios/spdx/compare/0.10.2...0.10.3
[0.10.2]: https://github.com/EmbarkStudios/spdx/compare/0.10.1...0.10.2
[0.10.1]: https://github.com/EmbarkStudios/spdx/compare/0.10.0...0.10.1
[0.10.0]: https://github.com/EmbarkStudios/spdx/compare/0.9.0...0.10.0
[0.9.0]: https://github.com/EmbarkStudios/spdx/compare/0.8.1...0.9.0
[0.8.1]: https://github.com/EmbarkStudios/spdx/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/EmbarkStudios/spdx/compare/0.7.0...0.8.0
[0.7.0]: https://github.com/EmbarkStudios/spdx/compare/0.6.2...0.7.0
[0.6.2]: https://github.com/EmbarkStudios/spdx/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/EmbarkStudios/spdx/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/EmbarkStudios/spdx/compare/0.5.0...0.6.0
[0.5.0]: https://github.com/EmbarkStudios/spdx/compare/0.4.1...0.5.0
[0.4.1]: https://github.com/EmbarkStudios/spdx/compare/0.4.0...0.4.1
[0.4.0]: https://github.com/EmbarkStudios/spdx/compare/0.3.6...0.4.0
[0.3.6]: https://github.com/EmbarkStudios/spdx/compare/0.3.5...0.3.6
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
