use spdx::ParseError;

macro_rules! test_validate {
    (ok [$($text:expr => [$($expected:expr),+$(,)?]),+$(,)?]) => {
        $(
            let val_expr = spdx::Expression::parse($text).unwrap();
            let mut reqs = val_expr.requirements().enumerate();

            $(
                let actual = reqs.next().unwrap();
                let actual_str = format!("{}", actual.1.req);
                let expected_str = $expected;

                similar_asserts::assert_eq!(actual_str, expected_str, "failed @ index {}", actual.0);
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
            original: $text.to_owned(),
            span: $range,
            reason: spdx::error::Reason::$reason,
        };

        similar_asserts::assert_eq!(act_err, expected);
    };

    ($text:expr => $unexpected:expr; $range:expr) => {
        let act_err = spdx::Expression::parse($text).unwrap_err();

        let expected = ParseError {
            original: $text.to_owned(),
            span: $range,
            reason: spdx::error::Reason::Unexpected($unexpected),
        };

        similar_asserts::assert_eq!(act_err, expected);
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
    err!("Apache-2.0 WITH WITH LLVM-exception OR Apache-2.0" => &["<addition>"]; 16..20);
    err!("Apache-2.0 WITH WITH LLVM-exception" => &["<addition>"]; 16..20);
    err!("(Apache-2.0) WITH LLVM-exception" => &["AND", "OR"]; 13..17);
    err!("Apache-2.0 (WITH LLVM-exception)" => &["AND", "OR", "WITH", ")", "+"]; 11..12);
    err!("(Apache-2.0 WITH) LLVM-exception" => &["<addition>"]; 16..17);
    err!("(Apache-2.0 WITH)+ LLVM-exception" => &["<addition>"]; 16..17);
    err!("Apache-2.0 WITH MIT" => &["<addition>"]; 16..19);
    err!("Apache-2.0 WITH WITH MIT" => &["<addition>"]; 16..20);
    err!("Apache-2.0 AND WITH MIT" => &["<license>", "("]; 15..19);
    err!("Apache-2.0 WITH AND MIT" => &["<addition>"]; 16..19);
    err!("Apache-2.0 WITH" => &["<addition>"]; 15..15);
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
    err!("LAL-1.2 WITH+ LLVM-exception" => &["<addition>"]; 12..13);
    err!("LAL-1.2 WITH LLVM-exception+" => &["AND", "OR", ")"]; 27..28);
    err!("LAL-1.2 WITH AdditionRef-myexc+" => &["AND", "OR", ")"]; 30..31);
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
    err!("MIT-advertising WITH AND" => &["<addition>"]; 21..24);
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
    let addition_ref = "MPL-2.0 WITH AdditionRef-Embark-Exception";
    let doc_addition_ref = "MIT WITH DocumentRef-Embark:AdditionRef-Embark-Exception";

    test_validate!(ok [
        with_exception => [with_exception],
        addition_ref => [addition_ref],
        doc_addition_ref => [doc_addition_ref],
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
    let expression = "(MIT AND (BitTorrent-1.1+ OR BSD-3-Clause))";

    test_validate!(ok [
        expression => [
            "MIT",
            "BitTorrent-1.1+",
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

#[test]
fn canonicalization() {
    use spdx::Expression;

    assert!(
        Expression::canonicalize("Apache-2.0 OR MIT")
            .unwrap()
            .is_none()
    );

    macro_rules! canon {
        ($bad:literal, $exp:literal) => {
            assert_eq!(Expression::canonicalize($bad).unwrap().unwrap(), $exp);
        };
    }

    canon!("Apache-2.0/MIT", "Apache-2.0 OR MIT");
    canon!("MIT and GPL-3.0+", "MIT AND GPL-3.0-or-later");
    canon!("GPL-2.0 and mit", "GPL-2.0-only AND MIT");
    canon!(
        "simplified bsd license or gpl-2.0+",
        "BSD-2-Clause OR GPL-2.0-or-later"
    );
    canon!(
        "apache with LLVM-exception/mpl",
        "Apache-2.0 WITH LLVM-exception OR MPL-2.0"
    );
    canon!(
        "simplified bsd license or gpl-3.0+",
        "BSD-2-Clause OR GPL-3.0-or-later"
    );
    canon!(
        "apache with LLVM-exception/gpl-2.0",
        "Apache-2.0 WITH LLVM-exception OR GPL-2.0-only"
    );
    canon!(
        "apache with LLVM-exception / gpl-3.0 or mit",
        "Apache-2.0 WITH LLVM-exception OR GPL-3.0-only OR MIT"
    );
}
