use crate::{
    AdditionItem, LicenseItem, LicenseReq,
    error::{ParseError, Reason},
    lexer::{Lexer, Token},
};
use std::fmt;

/// A convenience wrapper for a license and optional additional text that can be
/// checked against a license requirement to see if it satisfies the requirement
/// placed by a license holder
///
/// ```
/// let licensee = spdx::Licensee::parse("GPL-2.0-or-later").unwrap();
/// let req = spdx::LicenseReq::from(spdx::license_id("GPL-2.0-or-later").unwrap());
///
/// assert!(licensee.satisfies(&req));
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub struct Licensee {
    inner: LicenseReq,
}

impl fmt::Display for Licensee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::str::FromStr for Licensee {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

impl Licensee {
    /// Creates a licensee from its component parts.
    ///
    /// Note that use of SPDX's `or_later` is completely ignored for licensees
    /// as it only applies to the license holder(s), not the licensee
    #[must_use]
    pub fn new(license: LicenseItem, addition: Option<AdditionItem>) -> Self {
        if let LicenseItem::Spdx { or_later, .. } = &license {
            debug_assert!(!or_later);
        }

        Self {
            inner: LicenseReq { license, addition },
        }
    }

    /// See [`Self::parse_mode`], this is a short-handle for `Licensee::parse_mode(.., ParseMode::STRICT)`.
    #[inline]
    pub fn parse(original: &str) -> Result<Self, ParseError> {
        Self::parse_mode(original, crate::ParseMode::STRICT)
    }

    /// Parses an simplified version of an SPDX license expression that can
    /// contain at most 1 valid SPDX license with an optional additional text
    /// joined by a `WITH`.
    ///
    /// ```
    /// use spdx::Licensee;
    ///
    /// // Normal single license
    /// Licensee::parse("MIT").unwrap();
    ///
    /// // SPDX allows license identifiers outside of the official license list
    /// // via the LicenseRef- prefix (with optional DocumentRef- prefix)
    /// Licensee::parse("LicenseRef-My-Super-Extra-Special-License").unwrap();
    /// Licensee::parse("DocumentRef-mydoc:LicenseRef-My-License").unwrap();
    ///
    /// // License and exception
    /// Licensee::parse("Apache-2.0 WITH LLVM-exception").unwrap();
    ///
    /// // SPDX allows license with additional text outside of the official
    /// // license exception list via the AdditionRef- prefix (with optional
    /// // DocumentRef- prefix)
    /// Licensee::parse("MIT WITH AdditionRef-My-Exception").unwrap();
    /// Licensee::parse("MIT WITH DocumentRef-mydoc:AdditionRef-My-Exception").unwrap();
    ///
    /// // `+` is only allowed to be used by license requirements from the license holder
    /// Licensee::parse("Apache-2.0+").unwrap_err();
    ///
    /// Licensee::parse_mode("GPL-2.0", spdx::ParseMode::LAX).unwrap();
    /// ```
    pub fn parse_mode(original: &str, mode: crate::ParseMode) -> Result<Self, ParseError> {
        let mut lexer = Lexer::new_mode(original, mode);

        let license = {
            let lt = lexer.next().ok_or_else(|| ParseError {
                original: original.to_owned(),
                span: 0..original.len(),
                reason: Reason::Empty,
            })??;

            match lt.token {
                Token::Spdx(id) => {
                    if !mode.allow_deprecated && id.is_deprecated() {
                        return Err(ParseError {
                            original: original.to_owned(),
                            span: lt.span,
                            reason: Reason::DeprecatedLicenseId,
                        });
                    }

                    LicenseItem::Spdx {
                        id,
                        or_later: false,
                    }
                }
                Token::LicenseRef { doc_ref, lic_ref } => LicenseItem::Other {
                    doc_ref: doc_ref.map(String::from),
                    lic_ref: lic_ref.to_owned(),
                },
                _ => {
                    return Err(ParseError {
                        original: original.to_owned(),
                        span: lt.span,
                        reason: Reason::Unexpected(&["<license>"]),
                    });
                }
            }
        };

        let addition = match lexer.next() {
            None => None,
            Some(lt) => {
                let lt = lt?;
                match lt.token {
                    Token::With => {
                        let lt = lexer.next().ok_or(ParseError {
                            original: original.to_owned(),
                            span: lt.span,
                            reason: Reason::Empty,
                        })??;

                        match lt.token {
                            Token::Exception(id) => Some(AdditionItem::Spdx(id)),
                            Token::AdditionRef { doc_ref, add_ref } => Some(AdditionItem::Other {
                                doc_ref: doc_ref.map(String::from),
                                add_ref: add_ref.to_owned(),
                            }),
                            _ => {
                                return Err(ParseError {
                                    original: original.to_owned(),
                                    span: lt.span,
                                    reason: Reason::Unexpected(&["<addition>"]),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError {
                            original: original.to_owned(),
                            span: lt.span,
                            reason: Reason::Unexpected(&["WITH"]),
                        });
                    }
                }
            }
        };

        Ok(Self {
            inner: LicenseReq { license, addition },
        })
    }

    /// Determines whether the specified license requirement is satisfied by
    /// this license (+addition)
    ///
    /// ```
    /// let licensee = spdx::Licensee::parse("Apache-2.0 WITH LLVM-exception").unwrap();
    ///
    /// assert!(licensee.satisfies(&spdx::LicenseReq {
    ///     license: spdx::LicenseItem::Spdx {
    ///         id: spdx::license_id("Apache-2.0").unwrap(),
    ///         // Means the license holder is fine with Apache-2.0 or higher
    ///         or_later: true,
    ///     },
    ///     addition: spdx::exception_id("LLVM-exception")
    ///         .map(spdx::AdditionItem::Spdx),
    /// }));
    /// ```
    #[must_use]
    pub fn satisfies(&self, req: &LicenseReq) -> bool {
        match (&self.inner.license, &req.license) {
            (LicenseItem::Spdx { id: a, .. }, LicenseItem::Spdx { id: b, or_later }) => {
                if a.index != b.index {
                    let version =
                        |s: &'static str| s.chars().all(|c| c == '.' || c.is_ascii_digit());

                    if *or_later {
                        let mut ai = a.name().split('-');
                        let mut bi = b.name().split('-');

                        loop {
                            match (ai.next(), bi.next()) {
                                (Some(a_comp), Some(b_comp)) => {
                                    if a_comp == b_comp {
                                        continue;
                                    }

                                    if version(a_comp) && version(b_comp) && a_comp > b_comp {
                                        continue;
                                    }

                                    return false;
                                }
                                (None, None) => {
                                    break;
                                }
                                _ => return false,
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }
            (
                LicenseItem::Other {
                    doc_ref: doc_a,
                    lic_ref: lic_a,
                },
                LicenseItem::Other {
                    doc_ref: doc_b,
                    lic_ref: lic_b,
                },
            ) => {
                if doc_a != doc_b || lic_a != lic_b {
                    return false;
                }
            }
            _ => return false,
        }

        req.addition == self.inner.addition
    }

    #[must_use]
    pub fn into_req(self) -> LicenseReq {
        self.inner
    }
}

impl PartialOrd<LicenseReq> for Licensee {
    #[inline]
    fn partial_cmp(&self, o: &LicenseReq) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(o)
    }
}

impl PartialEq<LicenseReq> for Licensee {
    #[inline]
    fn eq(&self, o: &LicenseReq) -> bool {
        self.inner.eq(o)
    }
}

impl AsRef<LicenseReq> for Licensee {
    #[inline]
    fn as_ref(&self) -> &LicenseReq {
        &self.inner
    }
}

#[cfg(test)]
mod test {
    use crate::{AdditionItem, LicenseItem, LicenseReq, Licensee, exception_id, license_id};

    const LICENSEES: &[&str] = &[
        "LicenseRef-Embark-Proprietary",
        "BSD-2-Clause",
        "Apache-2.0 WITH LLVM-exception",
        "BSD-2-Clause-FreeBSD",
        "BSL-1.0",
        "Zlib",
        "CC0-1.0",
        "FTL",
        "ISC",
        "MIT",
        "MPL-2.0",
        "BSD-3-Clause",
        "Unicode-DFS-2016",
        "Unlicense",
        "Apache-2.0",
        "Apache-2.0 WITH AdditionRef-Embark-Exception",
    ];

    #[test]
    fn handles_or_later() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse_mode(l, crate::ParseMode::LAX).unwrap())
            .collect();
        licensees.sort();

        let mpl_id = license_id("MPL-2.0").unwrap();
        let req = LicenseReq {
            license: LicenseItem::Spdx {
                id: mpl_id,
                or_later: true,
            },
            addition: None,
        };

        // Licensees can't have the `or_later`
        assert!(licensees.binary_search_by(|l| l.inner.cmp(&req)).is_err());

        match &licensees[licensees
            .binary_search_by(|l| l.partial_cmp(&req).unwrap())
            .unwrap()]
        .inner
        .license
        {
            LicenseItem::Spdx { id, .. } => assert_eq!(*id, mpl_id),
            o @ LicenseItem::Other { .. } => panic!("unexpected {o:?}"),
        }
    }

    #[test]
    fn handles_exceptions() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse_mode(l, crate::ParseMode::LAX).unwrap())
            .collect();
        licensees.sort();

        let apache_id = license_id("Apache-2.0").unwrap();
        let llvm_exc = exception_id("LLVM-exception").unwrap();
        let req = LicenseReq {
            license: LicenseItem::Spdx {
                id: apache_id,
                or_later: false,
            },
            addition: Some(AdditionItem::Spdx(llvm_exc)),
        };

        assert_eq!(
            &req,
            &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
        );
    }

    #[test]
    fn handles_license_ref() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse_mode(l, crate::ParseMode::LAX).unwrap())
            .collect();
        licensees.sort();

        let req = LicenseReq {
            license: LicenseItem::Other {
                doc_ref: None,
                lic_ref: "Embark-Proprietary".to_owned(),
            },
            addition: None,
        };

        assert_eq!(
            &req,
            &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
        );
    }

    #[test]
    fn handles_close() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse_mode(l, crate::ParseMode::LAX).unwrap())
            .collect();
        licensees.sort();

        for id in &["BSD-2-Clause", "BSD-2-Clause-FreeBSD"] {
            let lic_id = license_id(id).unwrap();
            let req = LicenseReq {
                license: LicenseItem::Spdx {
                    id: lic_id,
                    or_later: true,
                },
                addition: None,
            };

            // Licensees can't have the `or_later`
            assert!(licensees.binary_search_by(|l| l.inner.cmp(&req)).is_err());

            match &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
            .license
            {
                LicenseItem::Spdx { id, .. } => assert_eq!(*id, lic_id),
                o @ LicenseItem::Other { .. } => panic!("unexpected {o:?}"),
            }
        }
    }
}
