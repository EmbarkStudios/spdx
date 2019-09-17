use std::{cmp, error::Error, fmt};

mod identifiers;
mod lexer;
pub mod parser;

pub use lexer::{Lexer, Token};
pub use parser::ValidExpression;

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
    /// Indicates the license had a `+`, allowing the licensee to license
    /// the software under either the specific version, or any later versions
    pub or_later: bool,
}

impl<'a> fmt::Display for LicenseReq<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        self.license.fmt(f)?;
        if self.or_later {
            write!(f, "+")?;
        }
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
    SPDX(LicenseId),
    /// Either a documentref or licenseref, see https://spdx.org/spdx-specification-21-web-version#h.h430e9ypa0j9
    Other {
        document_ref: Option<&'a str>,
        license_ref: &'a str,
    },
}

impl<'a> fmt::Display for LicenseItem<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            LicenseItem::SPDX(s) => s.name.fmt(f),
            LicenseItem::Other {
                document_ref: Some(d),
                license_ref: l,
            } => write!(f, "DocumentRef-{}:LicenseRef-{}", d, l),
            LicenseItem::Other {
                document_ref: None,
                license_ref: l,
            } => write!(f, "LicenseRef-{}", l),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct AllowedLicense<'a> {
    pub license: LicenseItem<'a>,
    pub exception: Option<ExceptionId>,
}

impl<'a> AllowedLicense<'a> {
    pub fn parse(s: &'a str) -> Result<Self, ParseError<'a>> {
        let mut lexer = Lexer::new(s);

        let license = {
            let lt = lexer.next().ok_or_else(|| ParseError::Empty)??;

            match lt.token {
                Token::License(lic) => LicenseItem::SPDX(
                    license_id(lic).ok_or_else(|| ParseError::UnknownLicenseId(lic))?,
                ),
                Token::LicenseRef { doc, lic } => LicenseItem::Other {
                    document_ref: doc,
                    license_ref: lic,
                },
                _ => {
                    return Err(ParseError::UnexpectedToken(
                        &s[lt.start..lt.end],
                        &["<license_id>"],
                    ))
                }
            }
        };

        let exception = match lexer.next() {
            None => None,
            Some(lt) => {
                let lt = lt?;
                match lt.token {
                    Token::With => {
                        let lt = lexer.next().ok_or_else(|| ParseError::Empty)??;

                        match lt.token {
                            Token::Exception(exc) => Some(
                                exception_id(exc)
                                    .ok_or_else(|| ParseError::UnknownExceptionId(exc))?,
                            ),
                            _ => {
                                return Err(ParseError::UnexpectedToken(
                                    &s[lt.start..lt.end],
                                    &["<exception_id>"],
                                ))
                            }
                        }
                    }
                    _ => return Err(ParseError::UnexpectedToken(&s[lt.start..lt.end], &["WITH"])),
                }
            }
        };

        Ok(AllowedLicense { license, exception })
    }

    pub fn satisfies(&self, req: &LicenseReq<'_>) -> bool {
        match (&self.license, &req.license) {
            (LicenseItem::SPDX(a), LicenseItem::SPDX(b)) => {
                // TODO: Handle GPL shenanigans :-/
                if a.index != b.index {
                    if req.or_later {
                        // Most of the SPDX identifiers have end in `-<version number>`,
                        // so chop that off and ensure the base strings match, and if so,
                        // just a do a lexical compare, if this "allowed license" is >
                        // then we satisfed the license requirement
                        let a_name = &a.name[..a.name.rfind('-').unwrap_or_else(|| a.name.len())];
                        let b_name = &b.name[..b.name.rfind('-').unwrap_or_else(|| b.name.len())];

                        if a_name != b_name || a.name < b.name {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            }
            (
                LicenseItem::Other {
                    document_ref: doca,
                    license_ref: lica,
                },
                LicenseItem::Other {
                    document_ref: docb,
                    license_ref: licb,
                },
            ) => {
                if doca != docb || lica != licb {
                    return false;
                }
            }
            _ => return false,
        }

        req.exception == self.exception
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
