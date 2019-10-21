use std::{cmp, fmt};

pub mod error;
pub mod expression;
mod identifiers;
mod lexer;
mod licensee;
pub mod parser;

pub use error::ParseError;
pub use expression::Expression;
pub use identifiers::{IS_COPYLEFT, IS_DEPRECATED, IS_FSF_LIBRE, IS_OSI_APPROVED};
pub use lexer::{Lexer, Token};
pub use licensee::Licensee;

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

    /// Returns true if the license is [copyleft](https://en.wikipedia.org/wiki/Copyleft)
    #[inline]
    pub fn is_copyleft(self) -> bool {
        self.flags & IS_COPYLEFT != 0
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
pub struct LicenseReq {
    /// The license
    pub license: LicenseItem,
    /// The exception allowed for this license, as specified following
    /// the `WITH` operator
    pub exception: Option<ExceptionId>,
}

impl fmt::Display for LicenseReq {
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
#[derive(Debug, Clone, Eq, Ord)]
pub enum LicenseItem {
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
        doc_ref: Option<String>,
        /// Purpose: Provide a locally unique identifier to refer to licenses that are not found on the SPDX License List.
        /// https://spdx.org/spdx-specification-21-web-version#h.4f1mdlm
        lic_ref: String,
    },
}

impl PartialOrd for LicenseItem {
    fn partial_cmp(&self, o: &Self) -> Option<cmp::Ordering> {
        match (self, o) {
            (Self::SPDX { id: a, .. }, Self::SPDX { id: b, .. }) => a.partial_cmp(b),
            (
                Self::Other {
                    doc_ref: ad,
                    lic_ref: al,
                },
                Self::Other {
                    doc_ref: bd,
                    lic_ref: bl,
                },
            ) => match ad.cmp(bd) {
                cmp::Ordering::Equal => al.partial_cmp(bl),
                o => Some(o),
            },
            (Self::SPDX { .. }, Self::Other { .. }) => Some(cmp::Ordering::Less),
            (Self::Other { .. }, Self::SPDX { .. }) => Some(cmp::Ordering::Greater),
        }
    }
}

impl PartialEq for LicenseItem {
    fn eq(&self, o: &Self) -> bool {
        if let Some(cmp::Ordering::Equal) = self.partial_cmp(o) {
            true
        } else {
            false
        }
    }
}

impl fmt::Display for LicenseItem {
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

/// Returns the version number of the SPDX list from which
/// the license and exception identifiers are sourced from
#[inline]
pub fn license_version() -> &'static str {
    identifiers::VERSION
}
