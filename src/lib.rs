/// Error types
pub mod error;
mod expression;
/// Auto-generated lists of license identifiers and exception identifiers
pub mod identifiers;
/// Contains types for lexing an SPDX license expression
pub mod lexer;
mod licensee;

pub use error::ParseError;
pub use expression::Expression;
use identifiers::{IS_COPYLEFT, IS_DEPRECATED, IS_FSF_LIBRE, IS_GNU, IS_OSI_APPROVED};
pub use lexer::ParseMode;
pub use licensee::Licensee;
use std::{cmp, fmt};

/// Unique identifier for a particular license
///
/// ```
/// let bsd = spdx::license_id("BSD-3-Clause").unwrap();
///
/// assert!(
///     bsd.is_fsf_free_libre()
///     && bsd.is_osi_approved()
///     && !bsd.is_deprecated()
///     && !bsd.is_copyleft()
/// );
/// ```
#[derive(Copy, Clone, Eq, Ord)]
pub struct LicenseId {
    /// The short identifier for the license
    pub name: &'static str,
    /// The full name of the license
    pub full_name: &'static str,
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
    ///
    /// ```
    /// assert!(spdx::license_id("GPL-2.0-only").unwrap().is_fsf_free_libre());
    /// ```
    #[inline]
    pub fn is_fsf_free_libre(self) -> bool {
        self.flags & IS_FSF_LIBRE != 0
    }

    /// Returns true if the license is [OSI approved](https://opensource.org/licenses)
    ///
    /// ```
    /// assert!(spdx::license_id("MIT").unwrap().is_osi_approved());
    /// ```
    #[inline]
    pub fn is_osi_approved(self) -> bool {
        self.flags & IS_OSI_APPROVED != 0
    }

    /// Returns true if the license is deprecated
    ///
    /// ```
    /// assert!(spdx::license_id("wxWindows").unwrap().is_deprecated());
    /// ```
    #[inline]
    pub fn is_deprecated(self) -> bool {
        self.flags & IS_DEPRECATED != 0
    }

    /// Returns true if the license is [copyleft](https://en.wikipedia.org/wiki/Copyleft)
    ///
    /// ```
    /// assert!(spdx::license_id("LGPL-3.0-or-later").unwrap().is_copyleft());
    /// ```
    #[inline]
    pub fn is_copyleft(self) -> bool {
        self.flags & IS_COPYLEFT != 0
    }

    /// Returns true if the license is a [GNU license](https://www.gnu.org/licenses/identify-licenses-clearly.html),
    /// which operate differently than all other SPDX license identifiers
    ///
    /// ```
    /// assert!(spdx::license_id("AGPL-3.0-only").unwrap().is_gnu());
    /// ```
    #[inline]
    pub fn is_gnu(self) -> bool {
        self.flags & IS_GNU != 0
    }
}

impl fmt::Debug for LicenseId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Unique identifier for a particular exception
///
/// ```
/// let exception_id = spdx::exception_id("LLVM-exception").unwrap();
/// assert!(!exception_id.is_deprecated());
/// ```
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
    ///
    /// ```
    /// assert!(spdx::exception_id("Nokia-Qt-exception-1.1").unwrap().is_deprecated());
    /// ```
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
/// LicenseItem, and may allow current and future versions of the license,
/// and may also allow for a specific exception
///
/// While they can be constructed manually, most of the time these will
/// be parsed and combined in an `Expression`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LicenseReq {
    /// The license
    pub license: LicenseItem,
    /// The exception allowed for this license, as specified following
    /// the `WITH` operator
    pub exception: Option<ExceptionId>,
}

impl From<LicenseId> for LicenseReq {
    fn from(id: LicenseId) -> Self {
        let mut or_later = false;

        // We need to special case GNU licenses because reasons
        let id = if id.is_gnu() {
            let root = if id.name.ends_with("-or-later") {
                or_later = true;
                &id.name[..id.name.len() - 9]
            } else if id.name.ends_with("-only") {
                &id.name[..id.name.len() - 5]
            } else {
                &id.name[..]
            };

            // If the root, eg GPL-2.0 licenses, which are currently deprecated,
            // are actually removed we will need to add them manually, but that
            // should only occur on a major revision of the SPDX license list,
            // so for now we should be fine with this
            license_id(root).expect("Unable to find root GNU license")
        } else {
            id
        };

        Self {
            license: LicenseItem::SPDX { id, or_later },
            exception: None,
        }
    }
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
        /// See the [spec](https://spdx.org/spdx-specification-21-web-version#h.h430e9ypa0j9) for
        /// more details.
        doc_ref: Option<String>,
        /// Purpose: Provide a locally unique identifier to refer to licenses that are not found on the SPDX License List.
        /// See the [spec](https://spdx.org/spdx-specification-21-web-version#h.4f1mdlm) for
        /// more details.
        lic_ref: String,
    },
}

impl LicenseItem {
    /// Returns the license identifier, if it is a recognized SPDX license and not
    /// a license referencer
    pub fn id(&self) -> Option<LicenseId> {
        match self {
            Self::SPDX { id, .. } => Some(*id),
            _ => None,
        }
    }
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
///
/// ```
/// assert!(spdx::license_id("MIT").is_some());
/// assert!(spdx::license_id("BitTorrent-1.1+").is_some());
/// ```
#[inline]
pub fn license_id(name: &str) -> Option<LicenseId> {
    let name = name.trim_end_matches('+');
    identifiers::LICENSES
        .binary_search_by(|lic| lic.0.cmp(name))
        .map(|index| {
            let (name, full_name, flags) = identifiers::LICENSES[index];
            LicenseId {
                name,
                full_name,
                index,
                flags,
            }
        })
        .ok()
}

/// Find license partially matching the name, e.g. "apache" => "Apache-2.0"
/// Returns length (in bytes) of the string matched. Garbage at the end is
/// ignored. See
/// [identifiers::IMPRECISE_NAMES](identifiers/constant.IMPRECISE_NAMES.html)
/// for the list of invalid names, and the valid license identifiers they are
/// paired with.
///
/// ```
/// assert!(spdx::imprecise_license_id("simplified bsd license").unwrap().0 == spdx::license_id("BSD-2-Clause").unwrap());
/// ```
#[inline]
pub fn imprecise_license_id(name: &str) -> Option<(LicenseId, usize)> {
    for (prefix, correct_name) in identifiers::IMPRECISE_NAMES {
        if let Some(name_prefix) = name.as_bytes().get(0..prefix.len()) {
            if prefix.as_bytes().eq_ignore_ascii_case(name_prefix) {
                let mut len = prefix.len();
                if name.as_bytes().get(len).copied() == Some(b'+') {
                    len += 1;
                }
                return license_id(correct_name).map(|lic| (lic, len));
            }
        }
    }
    None
}

/// Attempts to find an ExceptionId for the string
///
/// ```
/// assert!(spdx::exception_id("LLVM-exception").is_some());
/// ```
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
///
/// ```
/// assert_eq!(spdx::license_version(), "3.8");
/// ```
#[inline]
pub fn license_version() -> &'static str {
    identifiers::VERSION
}
