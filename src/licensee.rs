use crate::{
    error::{ParseError, Reason},
    Lexer, LicenseItem, LicenseReq, Token, ExceptionId,
};

/// A convenience wrapper for a license and optional exception
/// that can be checked against a license requirement to see
/// if it satisfies/matches the requirement
#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Licensee {
    inner: LicenseReq,
}

impl Licensee {
    /// Creates a licensee from its component parts. Note that use of SPDX's
    /// `or_later` is completely ignore for licensees as it only applies
    /// to the license holder
    pub fn new(license: LicenseItem, exception: Option<ExceptionId>) -> Self {
        if let LicenseItem::SPDX { or_later, .. } = &license { debug_assert!(!or_later) }

        Self {
            inner: LicenseReq { license, exception }
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
