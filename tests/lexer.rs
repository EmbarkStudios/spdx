use spdx::{Lexer, ParseError, Token};

#[test]
fn lexes_all_the_things() {
    let text = "MIT OR + () Apache-2.0 WITH AND LicenseRef-World Classpath-exception-2.0 DocumentRef-Test:LicenseRef-Hello";
    let mut lexer = Lexer::new(text);
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::License("MIT"));
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::Or);
    assert_eq!(
        lexer.next().unwrap().unwrap_err(),
        ParseError::SeparatedPlus
    );
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::Plus);
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::OpenParen);
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::CloseParen);
    assert_eq!(
        lexer.next().unwrap().unwrap().token,
        Token::License("Apache-2.0")
    );
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::With);
    assert_eq!(lexer.next().unwrap().unwrap().token, Token::And);
    assert_eq!(
        lexer.next().unwrap().unwrap().token,
        Token::LicenseRef {
            doc: None,
            lic: "World"
        }
    );
    assert_eq!(
        lexer.next().unwrap().unwrap().token,
        Token::Exception("Classpath-exception-2.0")
    );
    assert_eq!(
        lexer.next().unwrap().unwrap().token,
        Token::LicenseRef {
            doc: Some("Test"),
            lic: "Hello"
        }
    );
    assert!(lexer.next().is_none());
}

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

#[test]
fn lexes_single() {
    let s = "0BSD";

    test_lex!(s, [Token::License(s)]);
}

#[test]
fn lexes_or() {
    let s = "Apache-2.0 OR MIT";

    test_lex!(
        s,
        [
            Token::License("Apache-2.0"),
            Token::Or,
            Token::License("MIT"),
        ]
    );
}

#[test]
fn lexes_exception() {
    let s = "Apache-2.0 WITH LLVM-exception";

    test_lex!(
        s,
        [
            Token::License("Apache-2.0"),
            Token::With,
            Token::Exception("LLVM-exception"),
        ]
    );
}

#[test]
fn lexes_exceptions_with_ors() {
    let s = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT";

    test_lex!(
        s,
        [
            Token::License("Apache-2.0"),
            Token::With,
            Token::Exception("LLVM-exception"),
            Token::Or,
            Token::License("Apache-2.0"),
            Token::Or,
            Token::License("MIT"),
        ]
    );
}

#[test]
fn lexes_and() {
    let s = "BSD-3-Clause AND Zlib";

    test_lex!(
        s,
        [
            Token::License("BSD-3-Clause"),
            Token::And,
            Token::License("Zlib"),
        ]
    );
}

#[test]
fn lexes_complex() {
    let complex = "(Apache-2.0 WITH LLVM-exception) OR Apache-2.0 OR MIT";

    test_lex!(
        complex,
        [
            Token::OpenParen,
            Token::License("Apache-2.0"),
            Token::With,
            Token::Exception("LLVM-exception"),
            Token::CloseParen,
            Token::Or,
            Token::License("Apache-2.0"),
            Token::Or,
            Token::License("MIT"),
        ]
    );
}
