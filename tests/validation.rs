use spdx::ParseError;

macro_rules! test_validate {
    (ok [$($text:expr => [$($expected:expr),+$(,)?]),+$(,)?]) => {
        $(
            let val_expr = spdx::Expression::parse($text).unwrap();
            let mut reqs = val_expr.requirements().enumerate();

            $(
                let actual = reqs.next().unwrap();
                println!("{:?}", actual);

                let actual_str = format!("{}", actual.1.req);
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

            if let Some((_, additional)) = reqs.next() {
                assert!(false, "found additional requirement {}", additional.req);
            }
        )+
    };
}

macro_rules! err {
    ($text:expr => $reason:ident @ $range:expr) => {
        let act_err = spdx::Expression::parse($text).unwrap_err();

        let expected = ParseError {
            original: $text,
            span: $range,
            reason: spdx::error::Reason::$reason,
        };

        if act_err != expected {
            let act_text = format!("{:?}", act_err);
            let exp_text = format!("{:?}", expected);
            assert!(
                false,
                "{}",
                difference::Changeset::new(&exp_text, &act_text, "")
            );
        }
    };

    ($text:expr => $unexpected:expr; $range:expr) => {
        let act_err = spdx::Expression::parse($text).unwrap_err();

        let expected = ParseError {
            original: $text,
            span: $range,
            reason: spdx::error::Reason::Unexpected($unexpected),
        };

        if act_err != expected {
            let act_text = format!("{:?}", act_err);
            let exp_text = format!("{:?}", expected);
            assert!(
                false,
                "{}",
                difference::Changeset::new(&exp_text, &act_text, "")
            );
        }
    };
}

#[test]
fn fails_empty() {
    err!("" => Empty @ 0..0);
    err!(" " => Empty @ 0..1);
    err!("\n\t\n" => Empty @ 0..3);
    err!("()" => &["<license>", "("]; 1..2);
    err!("( )" => &["<license>", "("]; 2..3);
    err!("(   )" => &["<license>", "("]; 4..5);
    err!("  ( )" => &["<license>", "("]; 4..5);
    err!("AND" => &["<license>", "("]; 0..3);
}

#[test]
fn fails_unbalanced_parens() {
    err!("(Apache-2.0" => UnclosedParens @ 0..1);
    err!("BSD-3-Clause-No-Nuclear-License)" => UnopenedParens @ 31..32);
    err!("((BSD-3-Clause-No-Nuclear-License OR MIT)" => UnclosedParens @ 0..1);
    err!("(BSD-3-Clause-No-Nuclear-License OR MIT) AND Glulxe)" => UnopenedParens @ 51..52);
    err!("Glulxe OR MIT) AND BSD-3-Clause-No-Nuclear-License" => UnopenedParens @ 13..14);
    err!("Glulxe OR ( MIT AND BSD-3-Clause-No-Nuclear-License" => UnclosedParens @ 10..11);
}

#[test]
fn fails_bad_exception() {
    err!("Apache-2.0 WITH WITH LLVM-exception OR Apache-2.0" => &["<exception>"]; 16..20);
    err!("Apache-2.0 WITH WITH LLVM-exception" => &["<exception>"]; 16..20);
    err!("(Apache-2.0) WITH LLVM-exception" => &["AND", "OR"]; 13..17);
    err!("Apache-2.0 (WITH LLVM-exception)" => &["AND", "OR", "WITH", ")", "+"]; 11..12);
    err!("(Apache-2.0 WITH) LLVM-exception" => &["<exception>"]; 16..17);
    err!("(Apache-2.0 WITH)+ LLVM-exception" => &["<exception>"]; 16..17);
    err!("Apache-2.0 WITH MIT" => &["<exception>"]; 16..19);
    err!("Apache-2.0 WITH WITH MIT" => &["<exception>"]; 16..20);
    err!("Apache-2.0 AND WITH MIT" => &["<license>", "("]; 15..19);
    err!("Apache-2.0 WITH AND MIT" => &["<exception>"]; 16..19);
    err!("Apache-2.0 WITH" => &["<exception>"]; 15..15);
}

#[test]
fn fails_bad_plus() {
    err!("LAL-1.2 +" => SeparatedPlus @ 7..8);
    err!("+LAL-1.2" => &["<license>", "("]; 0..1);
    err!("++LAL-1.2" => &["<license>", "("]; 0..1);
    err!("LAL+-1.2" => UnknownTerm @ 0..3);
    err!("LAL-+1.2" => UnknownTerm @ 0..4);
    err!("LAL-1.+2" => UnknownTerm @ 0..6);
    err!("LAL-1.2++" => &["AND", "OR", "WITH", ")"]; 8..9);
    // + can only be applied to valid SDPX short identifiers, not license/doc refs
    err!("LicenseRef-Nope+" => &["AND", "OR", "WITH", ")"]; 15..16);
    err!("LAL-1.2 AND+" => &["<license>", "("]; 11..12);
    err!("LAL-1.2 OR +" => SeparatedPlus @ 10..11);
    err!("LAL-1.2 WITH+ LLVM-exception" => &["<exception>"]; 12..13);
    err!("LAL-1.2 WITH LLVM-exception+" => &["AND", "OR", ")"]; 27..28);
}

#[test]
fn fails_bad_ops() {
    err!("MIT-advertising AND" => &["<license>", "("]; 19..19);
    err!("MIT-advertising OR MIT AND" => &["<license>", "("]; 26..26);
    err!("MIT-advertising OR OR MIT" => &["<license>", "("]; 19..21);
    err!("MIT-advertising OR AND MIT" => &["<license>", "("]; 19..22);
    err!("MIT-advertising AND OR MIT" => &["<license>", "("]; 20..22);
    err!("(MIT-advertising AND) MIT" => &["<license>", "("]; 20..21);
    err!("MIT-advertising (AND MIT)" => &["AND", "OR", "WITH", ")", "+"]; 16..17);
    err!("OR MIT-advertising" => &["<license>", "("]; 0..2);
    err!("MIT-advertising WITH AND" => &["<exception>"]; 21..24);
}

#[test]
fn validates_single() {
    test_validate!(ok [
        "MIT" => ["MIT"],
        "Apache-2.0" => ["Apache-2.0"],
    ]);
}

#[test]
fn validates_canonical() {
    let canonical = "Apache-2.0 OR MIT";
    let canonical_w_parens = "(Apache-2.0 OR MIT)";

    test_validate!(ok [
        canonical => ["Apache-2.0", "MIT"],
        canonical_w_parens => ["Apache-2.0", "MIT"],
    ]);
}

#[test]
fn validates_single_with_exception() {
    let with_exception = "Apache-2.0 WITH LLVM-exception";

    test_validate!(ok [
        with_exception => [with_exception],
    ]);
}

#[test]
fn validates_complex() {
    let complex = "(Apache-2.0 WITH LLVM-exception OR Apache-2.0) AND MIT";

    test_validate!(ok [
        complex => [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "MIT",
        ]
    ]);
}

#[test]
fn validates_parens_plus() {
    let expression = "(MIT AND (LGPL-2.1+ OR BSD-3-Clause))";

    test_validate!(ok [
        expression => [
            "MIT",
            "LGPL-2.1+",
            "BSD-3-Clause",
        ]
    ]);
}

#[test]
fn validates_leading_parens() {
    let leading_parens = "((Apache-2.0 WITH LLVM-exception) OR Apache-2.0) AND OpenSSL OR MIT";

    test_validate!(ok [
        leading_parens => [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ]
    ]);
}

#[test]
fn validates_trailing_parens() {
    let trailing_parens = "Apache-2.0 WITH LLVM-exception OR Apache-2.0 AND (OpenSSL OR MIT)";

    test_validate!(ok [
        trailing_parens => [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ]
    ]);
}

#[test]
fn validates_middle_parens() {
    let middle_parens = "Apache-2.0 WITH LLVM-exception OR (Apache-2.0 AND OpenSSL) OR MIT";

    test_validate!(ok [
        middle_parens => [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ]
    ]);
}

#[test]
fn validates_excessive_parens() {
    let excessive_parens =
        "((((Apache-2.0 WITH LLVM-exception) OR (Apache-2.0)) AND (OpenSSL)) OR (MIT))";

    test_validate!(ok [
        excessive_parens=> [
            "Apache-2.0 WITH LLVM-exception",
            "Apache-2.0",
            "OpenSSL",
            "MIT",
        ]
    ]);
}
