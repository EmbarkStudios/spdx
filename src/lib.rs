/// Error types
pub mod error;
pub mod expression;
/// Auto-generated lists of license identifiers and exception identifiers
pub mod identifiers;
/// Contains types for lexing an SPDX license expression
pub mod lexer;
mod licensee;
/// Auto-generated full canonical text of each license
#[cfg(feature = "text")]
pub mod text;

pub use error::ParseError;
pub use expression::Expression;
pub use lexer::ParseMode;
pub use licensee::Licensee;
use std::{
    cmp::{self, Ordering},
    fmt,
};

pub mod flags {
    pub type Type = u8;

    /// Whether the license is listed as free by the [Free Software Foundation](https://www.gnu.org/licenses/license-list.en.html)
    pub const IS_FSF_LIBRE: Type = 0x1;
    /// Whether the license complies with the Open Source Definition as determined by the [Open Source Initiative](https://opensource.org/licenses)
    pub const IS_OSI_APPROVED: Type = 0x2;
    /// Whether the license or exception has been deprecated and should no longer be used
    pub const IS_DEPRECATED: Type = 0x4;
    /// Whether the license is considered copyleft
    pub const IS_COPYLEFT: Type = 0x8;
    /// Whether the license is a GNU license
    pub const IS_GNU: Type = 0x10;
}

/// An SPDX license
pub struct License {
    /// The short identifier for the license
    pub name: &'static str,
    /// The full name of the license
    pub full_name: &'static str,
    /// The index in the full license list where this license is positioned
    pub index: usize,
    /// The flags for this license
    pub flags: flags::Type,
}

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
#[derive(Copy, Clone)]
pub struct LicenseId {
    l: &'static License,
}

impl PartialEq for LicenseId {
    #[inline]
    fn eq(&self, o: &Self) -> bool {
        self.l.index == o.l.index
    }
}

impl Eq for LicenseId {}

impl Ord for LicenseId {
    #[inline]
    fn cmp(&self, o: &Self) -> Ordering {
        self.l.index.cmp(&o.l.index)
    }
}

impl PartialOrd for LicenseId {
    #[inline]
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl std::ops::Deref for LicenseId {
    type Target = License;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.l
    }
}

impl LicenseId {
    /// Returns true if the license is [considered free by the FSF](https://www.gnu.org/licenses/license-list.en.html)
    ///
    /// ```
    /// assert!(spdx::license_id("GPL-2.0-only").unwrap().is_fsf_free_libre());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_fsf_free_libre(self) -> bool {
        self.l.flags & flags::IS_FSF_LIBRE != 0
    }

    /// Returns true if the license is [OSI approved](https://opensource.org/licenses)
    ///
    /// ```
    /// assert!(spdx::license_id("MIT").unwrap().is_osi_approved());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_osi_approved(self) -> bool {
        self.l.flags & flags::IS_OSI_APPROVED != 0
    }

    /// Returns true if the license is deprecated
    ///
    /// ```
    /// assert!(spdx::license_id("wxWindows").unwrap().is_deprecated());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_deprecated(self) -> bool {
        self.l.flags & flags::IS_DEPRECATED != 0
    }

    /// Returns true if the license is [copyleft](https://en.wikipedia.org/wiki/Copyleft)
    ///
    /// ```
    /// assert!(spdx::license_id("LGPL-3.0-or-later").unwrap().is_copyleft());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_copyleft(self) -> bool {
        self.l.flags & flags::IS_COPYLEFT != 0
    }

    /// Returns true if the license is a [GNU license](https://www.gnu.org/licenses/identify-licenses-clearly.html),
    /// which operate differently than all other SPDX license identifiers
    ///
    /// ```
    /// assert!(spdx::license_id("AGPL-3.0-only").unwrap().is_gnu());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_gnu(self) -> bool {
        self.l.flags & flags::IS_GNU != 0
    }

    /// Retrieves the version of the license ID, if any
    ///
    /// ```
    /// assert_eq!(spdx::license_id("GPL-2.0-only").unwrap().version().unwrap(), "2.0");
    /// assert_eq!(spdx::license_id("BSD-3-Clause").unwrap().version().unwrap(), "3");
    /// assert!(spdx::license_id("Aladdin").unwrap().version().is_none());
    /// ```
    #[inline]
    pub fn version(self) -> Option<&'static str> {
        self.l.name
            .split('-')
            .find(|comp| comp.chars().all(|c| c == '.' || c.is_ascii_digit()))
    }

    /// The base name of the license
    ///
    /// ```
    /// assert_eq!(spdx::license_id("GPL-2.0-only").unwrap().base(), "GPL");
    /// assert_eq!(spdx::license_id("MIT").unwrap().base(), "MIT");
    /// ```
    #[inline]
    pub fn base(self) -> &'static str {
        self.l.name.split_once('-').map_or(self.l.name, |(n, _)| n)
    }

    /// Attempts to retrieve the license text
    ///
    /// ```
    /// assert!(spdx::license_id("GFDL-1.3-invariants").unwrap().text().contains("Invariant Sections"))
    /// ```
    #[cfg(feature = "text")]
    #[inline]
    pub fn text(self) -> &'static str {
        text::LICENSE_TEXTS[self.l.index].1
    }
}

impl fmt::Debug for LicenseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.l.name)
    }
}

/// An SPDX exception
pub struct Exception {
    /// The name of the exception
    pub name: &'static str,
    /// The index in the full exception list where this exception is positioned
    pub index: usize,
    /// The flags for the exception
    pub flags: flags::Type,
}

/// Unique identifier for a particular exception
///
/// ```
/// let exception_id = spdx::exception_id("LLVM-exception").unwrap();
/// assert!(!exception_id.is_deprecated());
/// ```
#[derive(Copy, Clone)]
pub struct ExceptionId {
    e: &'static Exception,
}

impl PartialEq for ExceptionId {
    #[inline]
    fn eq(&self, o: &Self) -> bool {
        self.e.index == o.e.index
    }
}

impl Eq for ExceptionId {}

impl Ord for ExceptionId {
    #[inline]
    fn cmp(&self, o: &Self) -> Ordering {
        self.e.index.cmp(&o.e.index)
    }
}

impl PartialOrd for ExceptionId {
    #[inline]
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        Some(self.cmp(o))
    }
}

impl std::ops::Deref for ExceptionId {
    type Target = Exception;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.e
    }
}

impl ExceptionId {
    /// Returns true if the exception is deprecated
    ///
    /// ```
    /// assert!(spdx::exception_id("Nokia-Qt-exception-1.1").unwrap().is_deprecated());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_deprecated(self) -> bool {
        self.e.flags & flags::IS_DEPRECATED != 0
    }

    /// Attempts to retrieve the license exception text
    ///
    /// ```
    /// assert!(spdx::exception_id("LLVM-exception").unwrap().text().contains("LLVM Exceptions to the Apache 2.0 License"));
    /// ```
    #[cfg(feature = "text")]
    #[inline]
    pub fn text(self) -> &'static str {
        text::EXCEPTION_TEXTS[self.e.index].1
    }
}

impl fmt::Debug for ExceptionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.e.name)
    }
}

/// Represents a single license requirement.
///
/// The requirement must include a valid [`LicenseItem`], and may allow current
/// and future versions of the license, and may also allow for a specific exception
///
/// While they can be constructed manually, most of the time these will
/// be parsed and combined in an [Expression]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LicenseReq {
    /// The license
    pub license: LicenseItem,
    /// The additional text for this license, as specified following
    /// the `WITH` operator
    pub addition: Option<AdditionItem>,
}

impl From<LicenseId> for LicenseReq {
    fn from(id: LicenseId) -> Self {
        Self {
            license: LicenseItem::Spdx {
                id,
                or_later: false,
            },
            addition: None,
        }
    }
}

impl fmt::Display for LicenseReq {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.license.fmt(f)?;

        if let Some(ref exe) = self.addition {
            write!(f, " WITH {exe}")?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct LicenseRef {
    /// Purpose: Identify any external SPDX documents referenced within this SPDX document.
    /// See the [spec](https://spdx.org/spdx-specification-21-web-version#h.h430e9ypa0j9) for
    /// more details.
    pub doc_ref: Option<String>,
    /// Purpose: Provide a locally unique identifier to refer to licenses that are not found on the SPDX License List.
    /// See the [spec](https://spdx.org/spdx-specification-21-web-version#h.4f1mdlm) for
    /// more details.
    pub lic_ref: String,
}

impl fmt::Display for LicenseRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match (&self.doc_ref, &self.lic_ref) {
            (Some(d), a) => write!(f, "DocumentRef-{d}:LicenseRef-{a}"),
            (None, a) => write!(f, "LicenseRef-{a}"),
        }
    }
}

/// A single license term in a license expression, according to the SPDX spec.
///
/// This can be either an SPDX license, which is mapped to a [`LicenseId`] from
/// a valid SPDX short identifier, or else a document and/or license ref
#[derive(Debug, Clone)]
pub enum LicenseItem {
    /// A regular SPDX license id
    Spdx {
        id: LicenseId,
        /// Indicates the license had a `+`, allowing the licensee to license
        /// the software under either the specific version, or any later versions
        or_later: bool,
    },
    Other(Box<LicenseRef>),
}

impl LicenseItem {
    /// Returns the license identifier, if it is a recognized SPDX license and not
    /// a license referencer
    #[must_use]
    pub fn id(&self) -> Option<LicenseId> {
        match self {
            Self::Spdx { id, .. } => Some(*id),
            Self::Other { .. } => None,
        }
    }
}

impl Ord for LicenseItem {
    fn cmp(&self, o: &Self) -> Ordering {
        match (self, o) {
            (
                Self::Spdx {
                    id: a,
                    or_later: la,
                },
                Self::Spdx {
                    id: b,
                    or_later: lb,
                },
            ) => match a.cmp(b) {
                Ordering::Equal => la.cmp(lb),
                o => o,
            },
            (Self::Other(a), Self::Other(b)) => a.cmp(b),
            (Self::Spdx { .. }, Self::Other { .. }) => Ordering::Less,
            (Self::Other { .. }, Self::Spdx { .. }) => Ordering::Greater,
        }
    }
}

impl PartialOrd for LicenseItem {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        match (self, o) {
            (Self::Spdx { id: a, .. }, Self::Spdx { id: b, .. }) => a.partial_cmp(b),
            (Self::Other(a), Self::Other(b)) => a.partial_cmp(b),
            (Self::Spdx { .. }, Self::Other { .. }) => Some(cmp::Ordering::Less),
            (Self::Other { .. }, Self::Spdx { .. }) => Some(cmp::Ordering::Greater),
        }
    }
}

impl PartialEq for LicenseItem {
    fn eq(&self, o: &Self) -> bool {
        matches!(self.partial_cmp(o), Some(cmp::Ordering::Equal))
    }
}

impl Eq for LicenseItem {}

impl fmt::Display for LicenseItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            LicenseItem::Spdx { id, or_later } => {
                id.name.fmt(f)?;

                if *or_later {
                    if id.is_gnu() && id.is_deprecated() {
                        f.write_str("-or-later")?;
                    } else if !id.is_gnu() {
                        f.write_str("+")?;
                    }
                }

                Ok(())
            }
            LicenseItem::Other(refs) => refs.fmt(f),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AdditionRef {
    /// Purpose: Identify any external SPDX documents referenced within this SPDX document.
    /// See the [spec](https://spdx.org/spdx-specification-21-web-version#h.h430e9ypa0j9) for
    /// more details.
    pub doc_ref: Option<String>,
    /// Purpose: Provide a locally unique identifier to refer to additional text that are not found on the SPDX License List.
    /// See the [spec](https://spdx.org/spdx-specification-21-web-version#h.4f1mdlm) for
    /// more details.
    pub add_ref: String,
}

impl fmt::Display for AdditionRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match (&self.doc_ref, &self.add_ref) {
            (Some(d), a) => write!(f, "DocumentRef-{d}:AdditionRef-{a}"),
            (None, a) => write!(f, "AdditionRef-{a}"),
        }
    }
}

/// A single addition term in a addition expression, according to the SPDX spec.
///
/// This can be either an SPDX license exception, which is mapped to a [`ExceptionId`]
/// from a valid SPDX short identifier, or else a document and/or addition ref
#[derive(Debug, Clone)]
pub enum AdditionItem {
    /// A regular SPDX license exception id
    Spdx(ExceptionId),
    Other(Box<AdditionRef>),
}

impl AdditionItem {
    /// Returns the license exception identifier, if it is a recognized SPDX license exception
    /// and not a license exception referencer
    #[must_use]
    pub fn id(&self) -> Option<ExceptionId> {
        match self {
            Self::Spdx(id) => Some(*id),
            Self::Other { .. } => None,
        }
    }
}

impl Ord for AdditionItem {
    fn cmp(&self, o: &Self) -> Ordering {
        match (self, o) {
            (Self::Spdx(a), Self::Spdx(b)) => match a.cmp(b) {
                Ordering::Equal => a.cmp(b),
                o => o,
            },
            (Self::Other(a), Self::Other(b)) => a.cmp(b),
            (Self::Spdx(_), Self::Other { .. }) => Ordering::Less,
            (Self::Other { .. }, Self::Spdx(_)) => Ordering::Greater,
        }
    }
}

impl PartialOrd for AdditionItem {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, o: &Self) -> Option<Ordering> {
        match (self, o) {
            (Self::Spdx(a), Self::Spdx(b)) => a.partial_cmp(b),
            (Self::Other(a), Self::Other(b)) => a.partial_cmp(b),
            (Self::Spdx(_), Self::Other { .. }) => Some(cmp::Ordering::Less),
            (Self::Other { .. }, Self::Spdx(_)) => Some(cmp::Ordering::Greater),
        }
    }
}

impl PartialEq for AdditionItem {
    fn eq(&self, o: &Self) -> bool {
        matches!(self.partial_cmp(o), Some(cmp::Ordering::Equal))
    }
}
impl Eq for AdditionItem {}

impl fmt::Display for AdditionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            AdditionItem::Spdx(id) => id.name.fmt(f),
            AdditionItem::Other(refs) => refs.fmt(f),
        }
    }
}

/// Attempts to find a [`LicenseId`] given a short id.
///
/// Note that any `+` at the end is trimmed when searching for a match.
///
/// ```
/// assert!(spdx::license_id("MIT").is_some());
/// assert!(spdx::license_id("BitTorrent-1.1+").is_some());
/// ```
#[inline]
#[must_use]
pub fn license_id(name: &str) -> Option<LicenseId> {
    let name = name.trim_end_matches('+');
    identifiers::LICENSES
        .binary_search_by(|lic| lic.name.cmp(name))
        .map(|index| LicenseId { l: &identifiers::LICENSES[index] })
        .ok()
}

/// Attempts to find a GNU license from its base name.
///
/// GNU licenses are "special", unlike every other license in the SPDX list, they
/// have (in _most_ cases) a bare variant which is deprecated, eg. GPL-2.0, an
/// `-only` variant which acts like every other license, and an `-or-later`
/// variant which acts as if `+` was applied.
#[inline]
#[must_use]
pub fn gnu_license_id(base: &str, or_later: bool) -> Option<LicenseId> {
    if base.ends_with("-only") || base.ends_with("-or-later") {
        license_id(base)
    } else {
        let mut v = smallvec::SmallVec::<[u8; 32]>::new();
        v.resize(base.len() + if or_later { 9 } else { 5 }, 0);

        v[..base.len()].copy_from_slice(base.as_bytes());

        if or_later {
            v[base.len()..].copy_from_slice(b"-or-later");
        } else {
            v[base.len()..].copy_from_slice(b"-only");
        }

        let Ok(s) = std::str::from_utf8(v.as_slice()) else {
            // Unreachable, but whatever
            return None;
        };
        license_id(s)
    }
}

/// Find license partially matching the name, e.g. "apache" => "Apache-2.0"
///
/// Returns length (in bytes) of the string matched. Garbage at the end is
/// ignored. See [`crate::identifiers::IMPRECISE_NAMES`] for the list of invalid
/// names, and the valid license identifiers they are mapped to.
///
/// ```
/// assert_eq!(
///     spdx::imprecise_license_id("simplified bsd license").unwrap().0,
///     spdx::license_id("BSD-2-Clause").unwrap()
/// );
/// ```
#[inline]
#[must_use]
pub fn imprecise_license_id(name: &str) -> Option<(LicenseId, usize)> {
    for (prefix, correct_name) in identifiers::IMPRECISE_NAMES {
        if let Some(name_prefix) = name.as_bytes().get(0..prefix.len()) {
            if prefix.as_bytes().eq_ignore_ascii_case(name_prefix) {
                return license_id(correct_name).map(|lic| (lic, prefix.len()));
            }
        }
    }
    None
}

/// Attempts to find an [`ExceptionId`] for the string
///
/// ```
/// assert!(spdx::exception_id("LLVM-exception").is_some());
/// ```
#[inline]
#[must_use]
pub fn exception_id(name: &str) -> Option<ExceptionId> {
    identifiers::EXCEPTIONS
        .binary_search_by(|exc| exc.name.cmp(name))
        .map(|index| ExceptionId { e: &identifiers::EXCEPTIONS[index] })
        .ok()
}

/// Returns the version number of the SPDX list from which
/// the license and exception identifiers are sourced from
#[inline]
#[must_use]
pub fn license_version() -> &'static str {
    identifiers::VERSION
}

#[cfg(test)]
mod test {
    use super::LicenseItem;

    use crate::{Expression, license_id};

    #[test]
    fn gnu_or_later_display() {
        let gpl_or_later = LicenseItem::Spdx {
            id: license_id("GPL-3.0").unwrap(),
            or_later: true,
        };

        let gpl_or_later_in_id = LicenseItem::Spdx {
            id: license_id("GPL-3.0-or-later").unwrap(),
            or_later: true,
        };

        let gpl_or_later_parsed = Expression::parse("GPL-3.0-or-later").unwrap();

        let non_gnu_or_later = LicenseItem::Spdx {
            id: license_id("Apache-2.0").unwrap(),
            or_later: true,
        };

        assert_eq!(gpl_or_later.to_string(), "GPL-3.0-or-later");
        assert_eq!(gpl_or_later_parsed.to_string(), "GPL-3.0-or-later");
        assert_eq!(gpl_or_later_in_id.to_string(), "GPL-3.0-or-later");
        assert_eq!(non_gnu_or_later.to_string(), "Apache-2.0+");
    }
}
