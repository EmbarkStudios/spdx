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
pub fn iter_expr(license_expr: &str) -> impl Iterator<Item = Result<LicenseExpr<'_>, ParseError>> {
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
