use crate::ParseError;
use lazy_static::lazy_static;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
    License(&'a str),
    Exception(&'a str),
    LicenseRef { doc: Option<&'a str>, lic: &'a str },
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
            Token::License(s) => s.len(),
            Token::Exception(e) => e.len(),
            Token::LicenseRef { doc, lic } => {
                doc.map_or(0, |d| {
                    // +1 is for the `:`
                    "DocumentRef-".len() + d.len() + 1
                }) + "LicenseRef-".len()
                    + lic.len()
            }
            Token::With => 4,
            Token::And => 3,
            Token::Or => 2,
            Token::Plus | Token::OpenParen | Token::CloseParen => 1,
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
                    } else if crate::license_id(&m.as_str()).is_some() {
                        Some(Ok(Token::License(m.as_str())))
                    } else if crate::exception_id(&m.as_str()).is_some() {
                        Some(Ok(Token::Exception(m.as_str())))
                    } else if let Some(c) = DOCREFLICREF.captures(m.as_str()) {
                        Some(Ok(Token::LicenseRef {
                            doc: Some(c.get(1).unwrap().as_str()),
                            lic: c.get(2).unwrap().as_str(),
                        }))
                    } else if let Some(c) = LICREF.captures(m.as_str()) {
                        Some(Ok(Token::LicenseRef {
                            doc: None,
                            lic: c.get(1).unwrap().as_str(),
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
