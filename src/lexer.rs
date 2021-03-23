use crate::{
    error::{ParseError, Reason},
    ExceptionId, LicenseId,
};
use lazy_static::lazy_static;

/// Available modes when parsing SPDX expressions
#[derive(Copy, Clone, PartialEq)]
pub enum ParseMode {
    /// Strict SPDX parsing.
    /// 1. Only license identifiers in the SPDX license list, or
    /// Document/LicenseRef, are allowed. The license identifiers are also
    /// case-sensitive.
    /// 1. `WITH`, `AND`, and `OR` are the only valid operators
    Strict,
    /// Allow non-conforming syntax for crates-io compatibility
    /// 1. Additional, invalid, identifiers are accepted and mapped to a correct
    /// SPDX license identifier. See
    /// [identifiers::IMPRECISE_NAMES](../identifiers/constant.IMPRECISE_NAMES.html)
    /// for the list of additionally accepted identifiers and the license they
    /// correspond to.
    /// 1. `/` can by used as a synonym for `OR`, and doesn't need to be
    /// separated by whitespace from the terms it combines
    Lax,
}

/// A single token in an SPDX license expression
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    /// A recognized SPDX license id
    SPDX(LicenseId),
    /// A `LicenseRef-` prefixed id, with an optional
    /// `DocRef-`
    LicenseRef {
        doc_ref: Option<&'a str>,
        lic_ref: &'a str,
    },
    /// A recognized SPDX exception id
    Exception(ExceptionId),
    /// A postfix `+` indicating "or later" for a particular SPDX license id
    Plus,
    /// A `(` for starting a group
    OpenParen,
    /// A `)` for ending a group
    CloseParen,
    /// A `WITH` operator
    With,
    /// An `AND` operator
    And,
    /// An `OR` operator
    Or,
}

impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

impl<'a> Token<'a> {
    fn len(&self) -> usize {
        match self {
            Token::SPDX(id) => id.name.len(),
            Token::Exception(e) => e.name.len(),
            Token::With => 4,
            Token::And => 3,
            Token::Or => 2,
            Token::Plus | Token::OpenParen | Token::CloseParen => 1,
            Token::LicenseRef { doc_ref, lic_ref } => {
                doc_ref.map_or(0, |d| {
                    // +1 is for the `:`
                    "DocumentRef-".len() + d.len() + 1
                }) + "LicenseRef-".len()
                    + lic_ref.len()
            }
        }
    }
}

/// Allows iteration through an SPDX license expression, yielding
/// a token or a `ParseError`.
///
/// Prefer to use `Expression::parse` or `Licensee::parse` rather
/// than directly using the lexer
pub struct Lexer<'a> {
    inner: &'a str,
    original: &'a str,
    offset: usize,
    lax: bool,
}

impl<'a> Lexer<'a> {
    /// Creates a Lexer over a license expression
    pub fn new(text: &'a str) -> Self {
        Self {
            inner: text,
            original: text,
            offset: 0,
            lax: false,
        }
    }

    /// Creates a Lexer over a license expression
    ///
    /// With `ParseMode::Lax` it allows non-conforming syntax
    /// used in crates-io crates.
    pub fn new_mode(text: &'a str, mode: ParseMode) -> Self {
        Self {
            inner: text,
            original: text,
            offset: 0,
            lax: mode == ParseMode::Lax,
        }
    }
}

/// A wrapper around a particular token that includes the span of the characters
/// in the original string, for diagnostic purposes
#[derive(Debug)]
pub struct LexerToken<'a> {
    /// The token that was lexed
    pub token: Token<'a>,
    /// The range of the token characters in the original license expression
    pub span: std::ops::Range<usize>,
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<LexerToken<'a>, ParseError<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        lazy_static! {
            static ref TEXTTOKEN: regex::Regex = regex::Regex::new(r"^[-a-zA-Z0-9.:]+").unwrap();
            static ref DOCREFLICREF: regex::Regex =
                regex::Regex::new(r"^DocumentRef-([-a-zA-Z0-9.]+):LicenseRef-([-a-zA-Z0-9.]+)")
                    .unwrap();
            static ref LICREF: regex::Regex =
                regex::Regex::new(r"^LicenseRef-([-a-zA-Z0-9.]+)").unwrap();
        }

        // Jump over any whitespace, updating `self.inner` and `self.offset` appropriately
        let non_whitespace_index = match self.inner.find(|c: char| !c.is_whitespace()) {
            Some(idx) => idx,
            None => self.inner.len(),
        };
        self.inner = &self.inner[non_whitespace_index..];
        self.offset += non_whitespace_index;

        #[allow(clippy::unnecessary_wraps)]
        fn ok_token<'a>(token: Token<'_>) -> Option<Result<(Token<'_>, usize), ParseError<'a>>> {
            let len = token.len();
            Some(Ok((token, len)))
        }

        match self.inner.chars().next() {
            None => None,
            // From SPDX 2.1 spec
            // There MUST NOT be whitespace between a license-id and any following "+".
            Some('+') => {
                if non_whitespace_index != 0 {
                    Some(Err(ParseError {
                        original: self.original,
                        span: self.offset - non_whitespace_index..self.offset,
                        reason: Reason::SeparatedPlus,
                    }))
                } else {
                    ok_token(Token::Plus)
                }
            }
            Some('(') => ok_token(Token::OpenParen),
            Some(')') => ok_token(Token::CloseParen),
            Some('/') if self.lax => Some(Ok((Token::Or, 1))),
            Some(_) => match TEXTTOKEN.find(self.inner) {
                None => Some(Err(ParseError {
                    original: self.original,
                    span: self.offset..self.offset + self.inner.len(),
                    reason: Reason::InvalidCharacters,
                })),
                Some(m) => {
                    if m.as_str() == "WITH" {
                        ok_token(Token::With)
                    } else if m.as_str() == "AND" {
                        ok_token(Token::And)
                    } else if m.as_str() == "OR" {
                        ok_token(Token::Or)
                    } else if self.lax && m.as_str() == "and" {
                        ok_token(Token::And)
                    } else if self.lax && m.as_str() == "or" {
                        ok_token(Token::Or)
                    } else if let Some(lic_id) = crate::license_id(m.as_str()) {
                        ok_token(Token::SPDX(lic_id))
                    } else if let Some(exc_id) = crate::exception_id(m.as_str()) {
                        ok_token(Token::Exception(exc_id))
                    } else if let Some(c) = DOCREFLICREF.captures(m.as_str()) {
                        ok_token(Token::LicenseRef {
                            doc_ref: Some(c.get(1).unwrap().as_str()),
                            lic_ref: c.get(2).unwrap().as_str(),
                        })
                    } else if let Some(c) = LICREF.captures(m.as_str()) {
                        ok_token(Token::LicenseRef {
                            doc_ref: None,
                            lic_ref: c.get(1).unwrap().as_str(),
                        })
                    } else if let Some((lic_id, token_len)) = if self.lax {
                        crate::imprecise_license_id(self.inner)
                    } else {
                        None
                    } {
                        Some(Ok((Token::SPDX(lic_id), token_len)))
                    } else {
                        Some(Err(ParseError {
                            original: self.original,
                            span: self.offset..self.offset + m.end(),
                            reason: Reason::UnknownTerm,
                        }))
                    }
                }
            },
        }
        .map(|res| {
            res.map(|(tok, len)| {
                let start = self.offset;
                self.inner = &self.inner[len..];
                self.offset += len;

                LexerToken {
                    token: tok,
                    span: start..self.offset,
                }
            })
        })
    }
}
