use crate::{
    error::{ParseError, Reason},
    ExceptionId, Lexer, LicenseItem, LicenseReq, Token,
};
use std::fmt;

/// A convenience wrapper for a license and optional exception
/// that can be checked against a license requirement to see
/// if it satisfies/matches the requirement
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Licensee {
    inner: LicenseReq,
}

impl fmt::Display for Licensee {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl Licensee {
    /// Creates a licensee from its component parts. Note that use of SPDX's
    /// `or_later` is completely ignore for licensees as it only applies
    /// to the license holder
    pub fn new(license: LicenseItem, exception: Option<ExceptionId>) -> Self {
        if let LicenseItem::SPDX { or_later, .. } = &license {
            debug_assert!(!or_later)
        }

        Self {
            inner: LicenseReq { license, exception },
        }
    }

    /// Parses an simplified version of an SPDX license expression that
    /// can contain at most 1 valid SDPX license with an optional exception
    /// joined by a WITH.
    ///
    /// eg `<license-id>` | `<license-id> WITH <exception-id>`
    pub fn parse(original: &str) -> Result<Self, ParseError<'_>> {
        let mut lexer = Lexer::new(original);

        let license = {
            let lt = lexer.next().ok_or_else(|| ParseError {
                original,
                span: 0..original.len(),
                reason: Reason::Empty,
            })??;

            match lt.token {
                Token::SPDX(id) => LicenseItem::SPDX {
                    id,
                    or_later: false,
                },
                Token::LicenseRef { doc_ref, lic_ref } => LicenseItem::Other {
                    doc_ref: doc_ref.map(String::from),
                    lic_ref: lic_ref.to_owned(),
                },
                _ => {
                    return Err(ParseError {
                        original,
                        span: lt.span,
                        reason: Reason::Unexpected(&["<license>"]),
                    })
                }
            }
        };

        let exception = match lexer.next() {
            None => None,
            Some(lt) => {
                let lt = lt?;
                match lt.token {
                    Token::With => {
                        let lt = lexer.next().ok_or_else(|| ParseError {
                            original,
                            span: lt.span,
                            reason: Reason::Empty,
                        })??;

                        match lt.token {
                            Token::Exception(exc) => Some(exc),
                            _ => {
                                return Err(ParseError {
                                    original,
                                    span: lt.span,
                                    reason: Reason::Unexpected(&["<exception>"]),
                                })
                            }
                        }
                    }
                    _ => {
                        return Err(ParseError {
                            original,
                            span: lt.span,
                            reason: Reason::Unexpected(&["WITH"]),
                        })
                    }
                }
            }
        };

        Ok(Licensee {
            inner: LicenseReq { license, exception },
        })
    }

    /// Determines whether the specified license requirement is satisfied by
    /// this license (+exception)
    pub fn satisfies(&self, req: &LicenseReq) -> bool {
        match (&self.inner.license, &req.license) {
            (LicenseItem::SPDX { id: a, .. }, LicenseItem::SPDX { id: b, or_later }) => {
                // TODO: Handle GPL shenanigans :-/
                if a.index != b.index {
                    if *or_later {
                        // Many of the SPDX identifiers end with `-<version number>`,
                        // so chop that off and ensure the base strings match, and if so,
                        // just a do a lexical compare, if this "allowed license" is >,
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
                    doc_ref: doca,
                    lic_ref: lica,
                },
                LicenseItem::Other {
                    doc_ref: docb,
                    lic_ref: licb,
                },
            ) => {
                if doca != docb || lica != licb {
                    return false;
                }
            }
            _ => return false,
        }

        req.exception == self.inner.exception
    }
}

impl PartialOrd<LicenseReq> for Licensee {
    fn partial_cmp(&self, o: &LicenseReq) -> Option<std::cmp::Ordering> {
        self.inner.partial_cmp(o)
    }
}

impl PartialEq<LicenseReq> for Licensee {
    fn eq(&self, o: &LicenseReq) -> bool {
        self.inner.eq(o)
    }
}

#[cfg(test)]
mod test {
    use crate::{exception_id, license_id, LicenseItem, LicenseReq, Licensee};

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
    ];

    #[test]
    fn handles_or_later() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        let mpl_id = license_id("MPL-2.0").unwrap();
        let req = LicenseReq {
            license: LicenseItem::SPDX {
                id: mpl_id,
                or_later: true,
            },
            exception: None,
        };

        // Licensees can't have the `or_later`
        assert!(licensees.binary_search_by(|l| l.inner.cmp(&req)).is_err());

        match &licensees[licensees
            .binary_search_by(|l| l.partial_cmp(&req).unwrap())
            .unwrap()]
        .inner
        .license
        {
            LicenseItem::SPDX { id, .. } => assert_eq!(*id, mpl_id),
            o => panic!("unexepcted {:?}", o),
        }
    }

    #[test]
    fn handles_exceptions() {
        let mut licensees: Vec<_> = LICENSEES
            .iter()
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        let apache_id = license_id("Apache-2.0").unwrap();
        let llvm_exc = exception_id("LLVM-exception").unwrap();
        let req = LicenseReq {
            license: LicenseItem::SPDX {
                id: apache_id,
                or_later: false,
            },
            exception: Some(llvm_exc),
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
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        let req = LicenseReq {
            license: LicenseItem::Other {
                doc_ref: None,
                lic_ref: "Embark-Proprietary".to_owned(),
            },
            exception: None,
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
            .map(|l| Licensee::parse(l).unwrap())
            .collect();
        licensees.sort();

        for id in &["BSD-2-Clause", "BSD-2-Clause-FreeBSD"] {
            let lic_id = license_id(id).unwrap();
            let req = LicenseReq {
                license: LicenseItem::SPDX {
                    id: lic_id,
                    or_later: true,
                },
                exception: None,
            };

            // Licensees can't have the `or_later`
            assert!(licensees.binary_search_by(|l| l.inner.cmp(&req)).is_err());

            match &licensees[licensees
                .binary_search_by(|l| l.partial_cmp(&req).unwrap())
                .unwrap()]
            .inner
            .license
            {
                LicenseItem::SPDX { id, .. } => assert_eq!(*id, lic_id),
                o => panic!("unexepcted {:?}", o),
            }
        }
    }
}
