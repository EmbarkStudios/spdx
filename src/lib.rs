use std::{error::Error, fmt};

mod identifiers;
mod lexer;

pub use lexer::{Lexer, Token};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ParseError<'a> {
    UnknownLicenseId(&'a str),
    UnknownExceptionId(&'a str),
    InvalidCharacters(&'a str),
    InvaldTerm(&'a str),
    UnbalancedParen(usize),
    /// A token did not match one of the expected tokens
    UnexpectedToken(&'a str, &'static [&'static str]),
    /// The expression is empty of significant tokens
    Empty,
    /// There was whitespace preceding a `+`
    SeparatedPlus,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            ParseError::UnknownLicenseId(info) => {
                format.write_fmt(format_args!("{}: {}", self.description(), info))
            }
            // ParseError::InvalidStructure(info) => {
            //     format.write_fmt(format_args!("{}: {}", self.description(), info))
            // }
            e => format.write_fmt(format_args!("OOPSIE WOOPSIE: {:?}", e)),
        }
    }
}

impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnknownLicenseId(_) => "unknown license or other term",
            //ParseError::InvalidStructure(_) => "invalid license expression",
            _ => unimplemented!(),
        }
    }
}

/// Unique identifier for a particular license
#[derive(Copy, Clone, Eq, Ord)]
pub struct LicenseId {
    /// The short identifier for the exception
    pub name: &'static str,
    index: usize,
    flags: u8,
}

impl PartialEq for LicenseId {
    #[inline]
    fn eq(&self, o: &LicenseId) -> bool {
        self.index == o.index
    }
}

impl PartialOrd for LicenseId {
    #[inline]
    fn partial_cmp(&self, o: &LicenseId) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&o.index)
    }
}

pub const IS_FSF_LIBRE: u8 = 0x1;
pub const IS_OSI_APPROVED: u8 = 0x2;
pub const IS_DEPRECATED: u8 = 0x4;

impl LicenseId {
    /// Returns true if the license is [considered free by the FSF](https://www.gnu.org/licenses/license-list.en.html)
    #[inline]
    pub fn is_fsf_free_libre(self) -> bool {
        self.flags & IS_FSF_LIBRE != 0
    }

    /// Returns true if the license is [OSI approved](https://opensource.org/licenses)
    #[inline]
    pub fn is_osi_approved(self) -> bool {
        self.flags & IS_OSI_APPROVED != 0
    }

    /// Returns true if the license is deprecated
    #[inline]
    pub fn is_deprecated(self) -> bool {
        self.flags & IS_DEPRECATED != 0
    }
}

impl fmt::Debug for LicenseId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Unique identifier for a particular exception
#[derive(Copy, Clone, Eq, Ord)]
pub struct ExceptionId {
    /// The short identifier for the exception
    pub name: &'static str,
    index: usize,
    flags: u8,
}

impl PartialEq for ExceptionId {
    #[inline]
    fn eq(&self, o: &ExceptionId) -> bool {
        self.index == o.index
    }
}

impl PartialOrd for ExceptionId {
    #[inline]
    fn partial_cmp(&self, o: &ExceptionId) -> Option<cmp::Ordering> {
        self.index.partial_cmp(&o.index)
    }
}

impl ExceptionId {
    /// Returns true if the exception is deprecated
    #[inline]
    pub fn is_deprecated(self) -> bool {
        self.flags & IS_DEPRECATED != 0
    }
}

impl fmt::Debug for ExceptionId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
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

/// Attempts to find a LicenseId for the string
/// Note: any '+' at the end is trimmed
#[inline]
pub fn license_id(name: &str) -> Option<LicenseId> {
    let name = &name.trim_end_matches('+');
    identifiers::LICENSES
        .binary_search_by(|lic| lic.0.cmp(name))
        .map(|index| {
            let (name, flags) = identifiers::LICENSES[index];
            LicenseId { name, index, flags }
        })
        .ok()
}

/// Attempts to find an ExceptionId for the string
#[inline]
pub fn exception_id(name: &str) -> Option<ExceptionId> {
    identifiers::EXCEPTIONS
        .binary_search_by(|exc| exc.0.cmp(name))
        .map(|index| {
            let (name, flags) = identifiers::EXCEPTIONS[index];
            ExceptionId { name, index, flags }
        })
        .ok()
}

/// Returns the version number of the SPDX set
#[inline]
pub fn license_version() -> &'static str {
    identifiers::VERSION
}
