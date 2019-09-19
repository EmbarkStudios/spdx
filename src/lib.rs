use std::{cmp, fmt};

mod allowed;
pub mod error;
mod identifiers;
mod lexer;
pub mod parser;

pub use allowed::AllowedLicense;
pub use error::ParseError;
pub use lexer::{Lexer, Token};
pub use parser::ValidExpression;

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

/// Represents a single license requirement, which must include a valid
/// LicenseItem, and may allow current a future versions of the license,
/// and may also allow for a specific exception
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LicenseReq<'a> {
    /// The license
    pub license: LicenseItem<'a>,
    /// The exception allowed for this license, as specified following
    /// `WITH`
    pub exception: Option<ExceptionId>,
}

impl<'a> fmt::Display for LicenseReq<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.license.fmt(f)?;

        if let Some(ref exe) = self.exception {
            write!(f, " WITH {}", exe.name)?;
        }
        Ok(())
    }
}

/// A single license term in a license expression, according to the SPDX spec.
/// This can be either an SPDX license, which is mapped to a LicenseId from
/// a valid SPDX short identifier, or else a document AND/OR license ref
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LicenseItem<'a> {
    /// A regular SPDX license id
    SPDX {
        id: LicenseId,
        /// Indicates the license had a `+`, allowing the licensee to license
        /// the software under either the specific version, or any later versions
        or_later: bool,
    },
    Other {
        /// Purpose: Identify any external SPDX documents referenced within this SPDX document.
        /// https://spdx.org/spdx-specification-21-web-version#h.h430e9ypa0j9
        doc_ref: Option<&'a str>,
        /// Purpose: Provide a locally unique identifier to refer to licenses that are not found on the SPDX License List.
        /// https://spdx.org/spdx-specification-21-web-version#h.4f1mdlm
        lic_ref: &'a str,
    },
}

impl<'a> fmt::Display for LicenseItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            LicenseItem::SPDX { id, or_later } => {
                id.name.fmt(f)?;

                if *or_later {
                    f.write_str("+")?;
                }

                Ok(())
            }
            LicenseItem::Other {
                doc_ref: Some(d),
                lic_ref: l,
            } => write!(f, "DocumentRef-{}:LicenseRef-{}", d, l),
            LicenseItem::Other {
                doc_ref: None,
                lic_ref: l,
            } => write!(f, "LicenseRef-{}", l),
        }
    }
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
