use spdx::lexer::{Lexer, Token};

macro_rules! test_lex {
    ($text:expr, [$($token:expr),+$(,)?]) => {
        let lexed: Vec<_> = Lexer::new($text).map(|r| r.map(|lt| lt.token).unwrap()).collect();
        let expected = {
            let mut v = Vec::new();
            $(
                v.push($token);
            )+
            v
        };

        assert_eq!(lexed, expected);
    }
}

macro_rules! lic_tok {
    ($id:expr) => {
        Token::SPDX(spdx::license_id($id).unwrap())
    };
}

macro_rules! exc_tok {
    ($id:expr) => {
        Token::Exception(spdx::exception_id($id).unwrap())
    };
}

#[test]
fn lexes_all_the_things() {
    let text = "MIT+ OR () Apache-2.0 WITH AND LicenseRef-World Classpath-exception-2.0 DocumentRef-Test:LicenseRef-Hello";

    test_lex!(
        text,
        [
            lic_tok!("MIT"),
            Token::Plus,
            Token::Or,
            Token::OpenParen,
            Token::CloseParen,
            lic_tok!("Apache-2.0"),
            Token::With,
            Token::And,
            Token::LicenseRef {
                doc_ref: None,
                lic_ref: "World",
            },
            exc_tok!("Classpath-exception-2.0"),
            Token::LicenseRef {
                doc_ref: Some("Test"),
                lic_ref: "Hello",
            },
        ]
    );
}

#[test]
fn lexes_single() {
    let s = "0BSD";

    test_lex!(s, [lic_tok!(s)]);
}

#[test]
fn lexes_or() {
    let s = "Apache-2.0 OR MIT";

    test_lex!(s, [lic_tok!("Apache-2.0"), Token::Or, lic_tok!("MIT"),]);
}

#[test]
fn lexes_exception() {
    let s = "Apache-2.0 WITH LLVM-exception";

    test_lex!(
        s,
        [
            lic_tok!("Apache-2.0"),
            Token::With,
            exc_tok!("LLVM-exception"),
        ]
    );
}

#[test]
fn lexes_exceptions_with_ors() {
    let s = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT";

    test_lex!(
        s,
        [
            lic_tok!("Apache-2.0"),
            Token::With,
            exc_tok!("LLVM-exception"),
            Token::Or,
            lic_tok!("Apache-2.0"),
            Token::Or,
            lic_tok!("MIT"),
        ]
    );
}

#[test]
fn lexes_and() {
    let s = "BSD-3-Clause AND Zlib";

    test_lex!(s, [lic_tok!("BSD-3-Clause"), Token::And, lic_tok!("Zlib"),]);
}

#[test]
fn fails_with_slash() {
    let mut lexer = Lexer::new("MIT/Apache-2.0");
    assert_eq!(lexer.next().unwrap().unwrap().token, lic_tok!("MIT"));
    assert_eq!(
        lexer.next().unwrap().unwrap_err(),
        spdx::ParseError {
            original: "MIT/Apache-2.0",
            span: 3..14,
            reason: spdx::error::Reason::InvalidCharacters,
        }
    );
}

#[test]
fn lax_takes_slash() {
    let lexed: Vec<_> = Lexer::new_mode("MIT/Apache", spdx::ParseMode::Lax)
        .map(|r| r.map(|lt| lt.token).unwrap())
        .collect();
    assert_eq!(
        &lexed,
        &[lic_tok!("MIT"), Token::Or, lic_tok!("Apache-2.0")]
    );
}

#[test]
fn fixes_license_names() {
    let lexed: Vec<_> = Lexer::new_mode("gpl v2 / bsd 2-clause", spdx::ParseMode::Lax)
        .map(|r| r.map(|lt| lt.token).unwrap())
        .collect();
    assert_eq!(
        &lexed,
        &[lic_tok!("GPL-2.0"), Token::Or, lic_tok!("BSD-2-Clause")]
    );
}

#[test]
fn lexes_complex() {
    let complex = "(Apache-2.0 WITH LLVM-exception) OR Apache-2.0 OR MIT";

    test_lex!(
        complex,
        [
            Token::OpenParen,
            lic_tok!("Apache-2.0"),
            Token::With,
            exc_tok!("LLVM-exception"),
            Token::CloseParen,
            Token::Or,
            lic_tok!("Apache-2.0"),
            Token::Or,
            lic_tok!("MIT"),
        ]
    );
}
