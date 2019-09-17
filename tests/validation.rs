use spdx::ParseError;

macro_rules! test_validate {
    (ok [$($text:expr; $stringified:expr => { $([$($expected:expr),+$(,)?]),+$(,)? }),+$(,)?]) => {
        $(
            let val_expr = spdx::ValidExpression::parse($text).unwrap();
            let stringified = format!("{}", val_expr);

            if stringified != $stringified {
                assert!(
                    false,
                    "{}",
                    difference::Changeset::new(&stringified, $stringified, " ")
                );
            }

            let mut licenses = val_expr.licenses().enumerate();

            $(
                $(
                    let actual = licenses.next().unwrap();
                    println!("{:?}", actual);

                    let actual_str = format!("{}", actual.1);
                    let expected_str = $expected;

                    if actual_str != expected_str {
                        assert!(
                            false,
                            "failed @ index {} - {}",
                            actual.0,
                            difference::Changeset::new(expected_str, &actual_str, " ")
                        );
                    }
                )+
            )+

            if let Some((_, additional)) = licenses.next() {
                assert!(false, "found additional requirement {}", additional);
            }
        )+
    };

    (err [$($text:expr => $expected:expr),+$(,)?]) => {
        $(
            let err = spdx::ValidExpression::parse($text).unwrap_err();

            if err != $expected {
                let act_text = format!("{}", err);
                let exp_text = format!("{}", $expected);
                assert!(
                    false,
                    "{}",
                    difference::Changeset::new(&exp_text, &act_text, "")
                );
            }
        )+
    };
}

#[test]
fn fails_empty() {
    test_validate!(err [
        "" => ParseError::Empty,
        " " => ParseError::Empty,
        "\n\t\n" => ParseError::Empty,
        "()" => ParseError::Empty,
        "(   )" => ParseError::Empty,
        "  (\n)" => ParseError::Empty,
    ]);
}

#[test]
fn fails_bad_exception() {
    test_validate!(err [
        "Apache-2.0 WITH+ LLVM-exception" => ParseError::UnexpectedToken("+", &["<license>"]),
        "Apache-2.0 WITH WITH LLVM-exception" => ParseError::UnexpectedToken("WITH", &["<exception>"]),
        //"(Apache-2.0) WITH LLVM-exception" => ParseError::Empty,
        "(Apache-2.0 WITH) LLVM-exception" => ParseError::UnexpectedToken(")", &["<exception>"]),
        "(Apache-2.0 WITH)+ LLVM-exception" => ParseError::UnexpectedToken(")", &["<exception>"]),
        //"Apache-2.0 (WITH LLVM-exception)" => ParseError::Empty,
        //"Apache-2.0 WITH MIT" => ParseError::UnexpectedToken("MIT", &["<exception>"]),
        //"Apache-2.0 OR WITH MIT" => ParseError::UnexpectedToken("WITH", &["<license>", "("]),
        //"Apache-2.0 WITH AND MIT" => ParseError::UnexpectedToken("AND", &["<exception>"]),
    ]);
}

#[test]
fn validates_single() {
    test_validate!(ok [
        "MIT"; "MIT" => { ["MIT"] },
        "Apache-2.0"; "Apache-2.0" => { ["Apache-2.0"] },
    ]);
}

#[test]
fn validates_canonical() {
    let canonical = "Apache-2.0 OR MIT";
    let canonical_w_parens = "(Apache-2.0 OR MIT)";

    test_validate!(ok [
        canonical; canonical => { ["Apache-2.0", "MIT"] },
        canonical_w_parens; canonical_w_parens => { ["Apache-2.0", "MIT"] },
    ]);
}

#[test]
fn validates_single_with_exception() {
    let with_exception = "Apache-2.0 WITH LLVM-exception";

    test_validate!(ok [
        with_exception; "Apache-2.0 WITH LLVM-exception" => { [with_exception] }
    ]);
}

#[test]
fn validates_complex() {
    let complex = "(Apache-2.0 WITH LLVM-exception OR Apache-2.0) AND MIT";

    test_validate!(ok [
        complex; complex => { [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "MIT",
        ] }
    ]);
}

#[test]
fn validates_parens_plus() {
    let expression = "(MIT AND (LGPL-2.1+ OR BSD-3-Clause))";

    test_validate!(ok [
        expression; expression => { [
            "MIT",
            "LGPL-2.1+",
            "BSD-3-Clause",
        ] }
    ]);
}

#[test]
fn validates_leading_parens() {
    let leading_parens = "((Apache-2.0 WITH LLVM-exception) OR Apache-2.0) AND OpenSSL OR MIT";

    test_validate!(ok [
        leading_parens; leading_parens => { [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ] }
    ]);
}

#[test]
fn validates_trailing_parens() {
    let trailing_parens = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 AND (OpenSSL OR MIT)";

    test_validate!(ok [
        trailing_parens; trailing_parens => { [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ] }
    ]);
}

#[test]
fn validates_middle_parens() {
    let middle_parens = "Apache-2.0 WITH LLVM-exception OR (Apache-2.0 AND OpenSSL) OR MIT";

    test_validate!(ok [
        middle_parens; middle_parens => { [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ] }
    ]);
}

#[test]
fn validates_excessive_parens() {
    let excessive_parens =
        "((((Apache-2.0 WITH LLVM-exception) OR (Apache-2.0)) AND (OpenSSL)) OR (MIT))";

    test_validate!(ok [
        excessive_parens; excessive_parens => { [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ] }
    ]);
}
