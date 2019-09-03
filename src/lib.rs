use std::{error::Error, fmt};

mod identifiers;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LicenseExpr {
    License(LicenseId),
    Exception(ExceptionId),
    And,
    Or,
    With,
}

use self::LicenseExpr::*;

impl fmt::Display for LicenseExpr {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            With => format.write_str("WITH"),
            And => format.write_str("AND"),
            Or => format.write_str("OR"),
            License(info) => format.write_str(info.name()),
            Exception(info) => format.write_str(info.name()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'a> {
    UnknownLicenseId(&'a str),
    InvalidStructure(LicenseExpr),
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ParseError::UnknownLicenseId(info) => {
                format.write_fmt(format_args!("{}: {}", self.description(), info))
            }
            ParseError::InvalidStructure(info) => {
                format.write_fmt(format_args!("{}: {}", self.description(), info))
            }
        }
    }
}

impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnknownLicenseId(_) => "unknown license or other term",
            ParseError::InvalidStructure(_) => "invalid license expression",
        }
    }
}

/// Iterates through the license and exception identifiers in an SPDX expression
pub fn iter_expr(license_expr: &str) -> impl Iterator<Item = Result<LicenseExpr, ParseError>> {
    license_expr.split_whitespace().map(|word| match word {
        "AND" => Ok(And),
        "OR" => Ok(Or),
        "WITH" => Ok(With),
        _ => {
            if let Some(id) = license_id(word) {
                Ok(License(id))
            } else if let Some(excid) = exception_id(word) {
                Ok(Exception(excid))
            } else {
                Err(ParseError::UnknownLicenseId(word))
            }
        }
    })
}

/// Unique identifier for a particular license
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LicenseId(usize);

const IS_FSF_LIBRE: u8 = 0x1;
const IS_OSI_APPROVED: u8 = 0x2;
const IS_DEPRECATED: u8 = 0x4;

impl LicenseId {
    /// The short identifier for the license
    #[inline]
    pub fn name(self) -> &'static str {
        identifiers::LICENSES[self.0].0
    }

    /// Returns true if the license is [considered free by the FSF](https://www.gnu.org/licenses/license-list.en.html)
    #[inline]
    pub fn is_fsf_free_libre(self) -> bool {
        identifiers::LICENSES[self.0].1 & IS_FSF_LIBRE != 0
    }

    /// Returns true if the license is [OSI approved](https://opensource.org/licenses)
    #[inline]
    pub fn is_osi_approved(self) -> bool {
        identifiers::LICENSES[self.0].1 & IS_OSI_APPROVED != 0
    }

    /// Returns true if the license is deprecated
    #[inline]
    pub fn is_deprecated(self) -> bool {
        identifiers::LICENSES[self.0].1 & IS_DEPRECATED != 0
    }
}

impl fmt::Debug for LicenseId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Unique identifier for a particular exception
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ExceptionId(usize);

impl ExceptionId {
    /// The short identifier for the exception
    #[inline]
    pub fn name(self) -> &'static str {
        identifiers::EXCEPTIONS[self.0].0
    }

    /// Returns true if the exception is deprecated
    #[inline]
    pub fn is_deprecated(self) -> bool {
        identifiers::EXCEPTIONS[self.0].1 & IS_DEPRECATED != 0
    }
}

impl fmt::Debug for ExceptionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Attempts to find a LicenseId for the string
/// Note: any '+' at the end is trimmed
#[inline]
pub fn license_id(name: &str) -> Option<LicenseId> {
    let name = &name.trim_end_matches('+');
    identifiers::LICENSES
        .binary_search_by(|lic| lic.0.cmp(name))
        .map(LicenseId)
        .ok()
}

/// Attempts to find an ExceptionId for the string
#[inline]
pub fn exception_id(name: &str) -> Option<ExceptionId> {
    identifiers::EXCEPTIONS
        .binary_search_by(|exc| exc.0.cmp(name))
        .map(ExceptionId)
        .ok()
}

/// Returns the version number of the SPDX set
#[inline]
pub fn license_version() -> &'static str {
    identifiers::VERSION
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parses_single() {
        let s = "0BSD";

        assert_eq!(
            iter_expr(s).map(|e| e.unwrap()).collect::<Vec<_>>(),
            vec![LicenseExpr::License(license_id(s).unwrap())]
        );
    }

    #[test]
    fn parses_or() {
        let s = "Apache-2.0 OR MIT";

        assert_eq!(
            iter_expr(s).map(|e| e.unwrap()).collect::<Vec<_>>(),
            vec![
                LicenseExpr::License(license_id("Apache-2.0").unwrap()),
                LicenseExpr::Or,
                LicenseExpr::License(license_id("MIT").unwrap()),
            ]
        );
    }

    #[test]
    fn parses_exception() {
        let s = "Apache-2.0 WITH LLVM-exception";

        assert_eq!(
            iter_expr(s).map(|e| e.unwrap()).collect::<Vec<_>>(),
            vec![
                LicenseExpr::License(license_id("Apache-2.0").unwrap()),
                LicenseExpr::With,
                LicenseExpr::Exception(exception_id("LLVM-exception").unwrap()),
            ]
        );
    }

    #[test]
    fn parses_exceptions_with_ors() {
        let s = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT";

        assert_eq!(
            iter_expr(s).map(|e| e.unwrap()).collect::<Vec<_>>(),
            vec![
                LicenseExpr::License(license_id("Apache-2.0").unwrap()),
                LicenseExpr::With,
                LicenseExpr::Exception(exception_id("LLVM-exception").unwrap()),
                LicenseExpr::Or,
                LicenseExpr::License(license_id("Apache-2.0").unwrap()),
                LicenseExpr::Or,
                LicenseExpr::License(license_id("MIT").unwrap()),
            ]
        );
    }

    #[test]
    fn parses_and() {
        let s = "BSD-3-Clause AND Zlib";

        assert_eq!(
            iter_expr(s).map(|e| e.unwrap()).collect::<Vec<_>>(),
            vec![
                LicenseExpr::License(license_id("BSD-3-Clause").unwrap()),
                LicenseExpr::And,
                LicenseExpr::License(license_id("Zlib").unwrap()),
            ]
        );
    }

    #[test]
    fn handles_deprecation() {
        assert!(license_id("GPL-3.0-with-autoconf-exception")
            .unwrap()
            .is_deprecated());
    }

    #[test]
    fn handles_fsf() {
        assert!(license_id("ZPL-2.1").unwrap().is_fsf_free_libre());
    }

    #[test]
    fn handles_osi() {
        assert!(license_id("RSCPL").unwrap().is_osi_approved());
    }

    #[test]
    fn handles_fsf_and_osi() {
        let id = license_id("Sleepycat").unwrap();

        assert!(id.is_fsf_free_libre() && id.is_osi_approved());
    }

    #[test]
    fn handles_deprecated_fsf_and_osi() {
        let id = license_id("LGPL-2.1+").unwrap();

        assert!(id.is_deprecated() && id.is_fsf_free_libre() && id.is_osi_approved());
    }
}
