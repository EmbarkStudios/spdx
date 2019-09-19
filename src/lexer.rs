use crate::{
    error::{ParseError, Reason},
    ExceptionId, LicenseItem,
};
use lazy_static::lazy_static;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    License(LicenseItem<'a>),
    Exception(ExceptionId),
    Plus,
    OpenParen,
    CloseParen,
    With,
    And,
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
            Token::License(LicenseItem::SPDX { id, .. }) => id.name.len(),
            Token::Exception(e) => e.name.len(),
            Token::With => 4,
            Token::And => 3,
            Token::Or => 2,
            Token::Plus | Token::OpenParen | Token::CloseParen => 1,
            Token::License(LicenseItem::Other { doc_ref, lic_ref }) => {
                doc_ref.map_or(0, |d| {
                    // +1 is for the `:`
                    "DocumentRef-".len() + d.len() + 1
                }) + "LicenseRef-".len()
                    + lic_ref.len()
            }
        }
    }
}

/// Allows iteration through a license expression, yielding
/// a token or a parser error
pub struct Lexer<'a> {
    inner: &'a str,
    offset: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a Lexer over a license expression
    pub fn new(text: &'a str) -> Self {
        Self {
            inner: text,
            offset: 0,
        }
    }
}

#[derive(Debug)]
pub struct LexerToken<'a> {
    /// The token that was lexed
    pub token: Token<'a>,
    /// The start index of the token in the original license expression
    pub start: usize,
    /// The end index of the token in the original license expression
    pub end: usize,
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

        match self.inner.chars().next() {
            None => None,
            Some('+') => Some(if non_whitespace_index != 0 {
                Err(ParseError::SeparatedPlus)
            } else {
                Ok(Token::Plus)
            }),
            Some('(') => Some(Ok(Token::OpenParen)),
            Some(')') => Some(Ok(Token::CloseParen)),
            _ => match TEXTTOKEN.find(self.inner) {
                None => Some(Err(ParseError::InvalidCharacters(self.inner))),
                Some(m) => {
                    if m.as_str() == "WITH" {
                        Some(Ok(Token::With))
                    } else if m.as_str() == "AND" {
                        Some(Ok(Token::And))
                    } else if m.as_str() == "OR" {
                        Some(Ok(Token::Or))
                    } else if let Some(lic_id) = crate::license_id(&m.as_str()) {
                        Some(Ok(Token::License(LicenseItem::SPDX {
                            id: lic_id,
                            or_later: false,
                        })))
                    } else if let Some(exc_id) = crate::exception_id(&m.as_str()) {
                        Some(Ok(Token::Exception(exc_id)))
                    } else if let Some(c) = DOCREFLICREF.captures(m.as_str()) {
                        Some(Ok(Token::License(LicenseItem::Other {
                            doc_ref: Some(c.get(1).unwrap().as_str()),
                            lic_ref: c.get(2).unwrap().as_str(),
                        })))
                    } else if let Some(c) = LICREF.captures(m.as_str()) {
                        Some(Ok(Token::License(LicenseItem::Other {
                            doc_ref: None,
                            lic_ref: c.get(1).unwrap().as_str(),
                        })))
                    } else {
                        }))
                    } else {
                        Some(Err(ParseError::InvaldTerm(m.as_str())))
                    }
                }
            },
        }
        .map(|res| {
            res.map(|tok| {
                let len = tok.len();
                let start = self.offset;
                self.inner = &self.inner[len..];
                self.offset += len;

                LexerToken {
                    token: tok,
                    start,
                    end: self.offset,
                }
            })
        })
    }
}
