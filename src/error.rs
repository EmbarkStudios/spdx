use std::{error::Error, fmt};

/// An error related to parsing of an SPDX license expression
/// or identifier
#[derive(Debug, PartialEq, Eq)]
pub struct ParseError {
    /// The string that was attempting to be parsed
    pub original: String,
    /// The range of characters in the original string that result
    /// in this error
    pub span: std::ops::Range<usize>,
    /// The specific reason for the error
    pub reason: Reason,
}

/// The particular reason for a `ParseError`
#[derive(Debug, PartialEq, Eq)]
pub enum Reason {
    /// The specified license short-identifier was not
    /// found the SPDX list
    UnknownLicense,
    /// The specified exception short-identifier was not
    /// found the SPDX list
    UnknownException,
    /// The characters are not valid in an SPDX license expression
    InvalidCharacters,
    /// An opening parens was unmatched with a closing parens
    UnclosedParens,
    /// A closing parens was unmatched with an opening parens
    UnopenedParens,
    /// The expression does not contain any valid terms
    Empty,
    /// Found an unexpected term, which wasn't one of the
    /// expected terms that is listed
    Unexpected(&'static [&'static str]),
    /// A + was found after whitespace, which is not allowed
    /// by the SPDX spec
    SeparatedPlus,
    /// When lexing, a term was found that was
    /// 1. Not a license short-id
    /// 2. Not an exception short-id
    /// 3. Not a document/license ref
    /// 4. Not an AND, OR, or WITH
    UnknownTerm,
    /// GNU suffix licenses don't allow `+` because they already have
    /// the `-or-later` suffix to denote that
    GnuNoPlus,
    /// A `+` was added to a GNU license that was specified as `-only` or `-or-later`
    GnuPlusWithSuffix,
    /// A deprecated license id was used
    DeprecatedLicenseId,
}

impl Reason {
    #[inline]
    fn description(&self) -> &'static str {
        match self {
            Reason::UnknownLicense => "unknown license id",
            Reason::UnknownException => "unknown exception id",
            Reason::InvalidCharacters => "invalid character(s)",
            Reason::UnclosedParens => "unclosed parens",
            Reason::UnopenedParens => "unopened parens",
            Reason::Empty => "empty expression",
            Reason::Unexpected(_) => "unexpected term",
            Reason::SeparatedPlus => "`+` must not follow whitespace",
            Reason::UnknownTerm => "unknown term",
            Reason::GnuNoPlus => "a GNU license was followed by a `+`",
            Reason::GnuPlusWithSuffix => {
                "a GNU license was followed by a `+` even though it ended in `-only` or `-or-later`"
            }
            Reason::DeprecatedLicenseId => "a deprecated license identifier was used",
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.original)?;
        f.write_str("\n")?;

        for _ in 0..self.span.start {
            f.write_str(" ")?;
        }

        // Mismatched parens have a slightly different output
        // than the other errors
        match &self.reason {
            Reason::UnclosedParens => f.write_fmt(format_args!("- {}", Reason::UnclosedParens)),
            Reason::UnopenedParens => f.write_fmt(format_args!("^ {}", Reason::UnopenedParens)),
            other => {
                for _ in self.span.start..self.span.end {
                    f.write_str("^")?;
                }

                f.write_fmt(format_args!(" {other}"))
            }
        }
    }
}

impl fmt::Display for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Self::Unexpected(expected) = self {
            if expected.len() > 1 {
                f.write_str("expected one of ")?;

                for (i, exp) in expected.iter().enumerate() {
                    f.write_fmt(format_args!("{}`{}`", if i > 0 { ", " } else { "" }, exp))?;
                }
                f.write_str(" here")
            } else if expected.is_empty() {
                f.write_str("the term was not expected here")
            } else {
                f.write_fmt(format_args!("expected a `{}` here", expected[0]))
            }
        } else {
            f.write_str(self.description())
        }
    }
}

impl Error for ParseError {
    #[inline]
    fn description(&self) -> &str {
        self.reason.description()
    }
}
