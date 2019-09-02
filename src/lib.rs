use std::{error::Error, fmt};

mod identifiers;

#[derive(Debug, Clone, Copy)]
pub enum LicenseExpr<'a> {
    License(&'a str),
    Exception(&'a str),
    And,
    Or,
    With,
}

use self::LicenseExpr::*;

impl<'a> fmt::Display for LicenseExpr<'a> {
    fn fmt(&self, format: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            With => format.write_str("WITH"),
            And => format.write_str("AND"),
            Or => format.write_str("OR"),
            License(info) | Exception(info) => format.write_str(info),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError<'a> {
    UnknownLicenseId(&'a str),
    InvalidStructure(LicenseExpr<'a>),
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
pub fn iter_expr(license_expr: &str) -> impl Iterator<Item = Result<LicenseExpr<'_>, ParseError>> {
    license_expr.split_whitespace().map(|word| match word {
        "AND" => Ok(And),
        "OR" => Ok(Or),
        "WITH" => Ok(With),
        _ if identifiers::LICENSES
            .binary_search(&word.trim_end_matches('+'))
            .is_ok() =>
        {
            Ok(License(word))
        }
        _ if identifiers::EXCEPTIONS.binary_search(&word).is_ok() => Ok(Exception(word)),
        _ => Err(ParseError::UnknownLicenseId(word)),
    })
}

/// Unique identifier for a particular license
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LicenseId(usize);

impl fmt::Debug for LicenseId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", license_name(*self))
    }
}

/// Attempts to find a LicenseId for the string
/// Note: any '+' at the end is trimmed
#[inline]
pub fn license_id(name: &str) -> Option<LicenseId> {
    identifiers::LICENSES
        .binary_search(&name.trim_end_matches('+'))
        .map(LicenseId)
        .ok()
}

/// Retrieves the string identifier for a license id
#[inline]
pub fn license_name(id: LicenseId) -> &'static str {
    identifiers::LICENSES[id.0]
}

/// Returns the version number of the SPDX set
pub fn license_version() -> &'static str {
    identifiers::VERSION
}
