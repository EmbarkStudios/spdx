#![allow(clippy::nonminimal_bool, clippy::eq_op, clippy::cognitive_complexity)]

use spdx::LicenseItem;

macro_rules! exact {
    ($req:expr, $e:expr) => {
        spdx::Licensee::parse($e).unwrap().satisfies($req)
    };
}

macro_rules! check {
    ($le:expr => [$($logical_expr:expr => $is_allowed:expr),+$(,)?]) => {
        check_mode!(spdx::ParseMode::Strict, $le => [$($logical_expr => $is_allowed),+])
    };
}

macro_rules! check_lax {
    ($le:expr => [$($logical_expr:expr => $is_allowed:expr),+$(,)?]) => {
        check_mode!(spdx::ParseMode::Lax, $le => [$($logical_expr => $is_allowed),+])
    };
}

macro_rules! check_mode {
    ($mode:expr, $le:expr => [$($logical_expr:expr => $is_allowed:expr),+$(,)?]) => {
        let validated = spdx::Expression::parse_mode($le, $mode).unwrap();

        $(
            // Evaluate the logical expression to determine if we are
            // expecting an Ok or Err
            let expected = $logical_expr;

            match validated.evaluate_with_failures($is_allowed) {
                Ok(_) => assert!(expected, "{} => {}", stringify!($logical_expr), stringify!($is_allowed)),
                Err(f) => assert!(!expected, "{} => {} {:?}", stringify!($logical_expr), stringify!($is_allowed), f),
            }
        )+
    };
}

#[test]
fn single_or() {
    check!("Apache-2.0 OR MIT" => [
        false || true => |req| exact!(req, "MIT"),
        true || false => |req| exact!(req, "Apache-2.0"),
        false || false => |req| exact!(req, "ISC"),
    ]);
}

#[test]
fn single_or_lax() {
    check_lax!("Apache/ MIT" => [
        false || true => |req| exact!(req, "MIT"),
        true || false => |req| exact!(req, "Apache-2.0"),
        false || false => |req| exact!(req, "ISC"),
    ]);
}

#[test]
fn single_and() {
    check!("Apache-2.0 AND MIT" => [
        false && true => |req| exact!(req, "MIT"),
        true && false => |req| exact!(req, "Apache-2.0"),
        false && false => |req| exact!(req, "ISC"),
        true && true => |req| exact!(req, "MIT") || exact!(req, "Apache-2.0"),
    ]);
}

#[test]
fn or_and() {
    check!("MIT OR Apache-2.0 AND BSD-2-Clause" => [
        true || true && false => |req| exact!(req, "MIT") || exact!(req, "Apache-2.0"),
        true || false && false => |req| exact!(req, "MIT"),
        false || false && true => |req| exact!(req, "BSD-2-Clause"),
        false || true && false => |req| exact!(req, "Apache-2.0"),
    ]);
}

#[test]
fn complex() {
    check!("(MIT AND (LGPL-2.1-or-later OR BSD-3-Clause))" => [
        false && (false || true) => |req| exact!(req, "MIT"),
        false && (false || false) => |req| exact!(req, "Apache-2.0"),
        true && (false || false) => |req| exact!(req, "MIT"),
        true && (false || true) => |req| exact!(req, "MIT") || exact!(req, "BSD-3-Clause"),
        true && (true || false) => |req| exact!(req, "MIT") || exact!(req, "LGPL-3.0"),
    ]);
}

#[test]
fn leading_parens() {
    check!("((Apache-2.0 WITH LLVM-exception) OR Apache-2.0) AND OpenSSL OR MIT" => [
        (false || false) && false || true => |req| exact!(req, "MIT"),
        (false || true) && false || false => |req| exact!(req, "Apache-2.0"),
        (false || true) && false || true => |req| exact!(req, "Apache-2.0") || exact!(req, "MIT"),
        (false || true) && true || false => |req| exact!(req, "Apache-2.0") || exact!(req, "OpenSSL"),
    ]);
}

#[test]
fn allow_trailing_parens() {
    check!("Apache-2.0 WITH LLVM-exception OR Apache-2.0 AND (OpenSSL OR MIT)" => [
        false || false && (false || true) => |req| exact!(req, "MIT"),
        true || false && (true || false) => |req| exact!(req, "Apache-2.0 WITH LLVM-exception") || exact!(req, "OpenSSL"),
        false || true && (false || true) => |req| exact!(req, "Apache-2.0") || exact!(req, "MIT"),
        false || false && (true || true) => |req| exact!(req, "MIT") || exact!(req, "OpenSSL"),
    ]);
}

#[test]
fn allow_middle_parens() {
    check!("Apache-2.0 WITH LLVM-exception OR (Apache-2.0 AND OpenSSL) OR MIT" => [
        false || (false && false) || true => |req| exact!(req, "MIT"),
        true || (false && false) || false => |req| exact!(req, "Apache-2.0 WITH LLVM-exception"),
        false || (true && false) || false => |req| exact!(req, "Apache-2.0"),
        false || (true && true) || false => |req| exact!(req, "Apache-2.0") || exact!(req, "OpenSSL"),
    ]);
}

#[test]
fn allow_excessive_parens() {
    check!("((((Apache-2.0 WITH LLVM-exception) OR (Apache-2.0)) AND (OpenSSL)) OR (MIT))" => [
        ((false || false) && false) || true => |req| exact!(req, "MIT"),
        ((true || false) && false) || false => |req| exact!(req, "Apache-2.0 WITH LLVM-exception"),
        ((false || true) && false) || false => |req| exact!(req, "Apache-2.0"),
        ((false || true) && true) || false => |req| exact!(req, "Apache-2.0") || exact!(req, "OpenSSL"),
    ]);
}

#[test]
fn allow_osi_fsf() {
    // Borceux is neither OSI or FSF
    // MIT is both
    // BitTorrent-1.1 is only FSF
    check!("Borceux OR MIT AND BitTorrent-1.1" => [
        false || true && true => |req| {
            if let LicenseItem::Spdx { id, .. } = req.license {
                return id.is_osi_approved() || id.is_fsf_free_libre();
            }

            false
        },
        false || true && false => |req| {
            if let LicenseItem::Spdx { id, .. } = req.license {
                return id.is_osi_approved() && id.is_fsf_free_libre();
            }

            false
        }
    ]);
}

#[test]
fn or_later() {
    check!("CC-BY-NC-ND-2.5+" => [
        false => |req| exact!(req, "CC-BY-NC-1.0"),
        false => |req| exact!(req, "CC-BY-NC-4.0"),
        false => |req| exact!(req, "CC-BY-NC-ND-1.0"),
        false => |req| exact!(req, "CC-BY-NC-ND-2.0"),
        true => |req| exact!(req, "CC-BY-NC-ND-2.5"),
        true => |req| exact!(req, "CC-BY-NC-ND-3.0"),
        true => |req| exact!(req, "CC-BY-NC-ND-4.0"),
        false => |req| exact!(req, "CC-BY-NC-SA-1.0"),
        false => |req| exact!(req, "CC-BY-NC-SA-4.0"),
    ]);
}

#[test]
fn lgpl_only() {
    check!("LGPL-2.1-only" => [
        false => |req| exact!(req, "LGPL-2.0"),
        true => |req| exact!(req, "LGPL-2.1"),
        false => |req| exact!(req, "LGPL-3.0"),
        //false => |req| exact!(req, "LGPL-4.0"),
    ]);
}

#[test]
fn gpl_or_later() {
    check!("GPL-3.0-or-later" => [
        false => |req| exact!(req, "GPL-1.0"),
        false => |req| exact!(req, "GPL-2.0"),
        true => |req| exact!(req, "GPL-3.0"),
        //true => |req| exact!(req, "GPL-4.0"),
    ]);
}

#[test]
fn gpl_or_later_plus_strict() {
    spdx::Expression::parse("GPL-2.0+").unwrap_err();
}

#[test]
fn gpl_or_later_plus_lax() {
    spdx::Expression::parse_mode("GPL-2.0+", spdx::ParseMode::Lax).unwrap();
}

#[test]
fn gfdl() {
    check!("GFDL-1.1-or-later" => [
        true => |req| exact!(req, "GFDL-1.1"),
        true => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.3"),
    ]);

    check!("GFDL-1.2-invariants-or-later" => [
        false => |req| exact!(req, "GFDL-1.1"),
        false => |req| exact!(req, "GFDL-1.1-invariants"),
        false => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.2-invariants"),
        false => |req| exact!(req, "GFDL-1.3"),
        true => |req| exact!(req, "GFDL-1.3-invariants"),
    ]);

    check_lax!("GFDL-1.1-invariants+" => [
        false => |req| exact!(req, "GFDL-1.1"),
        true => |req| exact!(req, "GFDL-1.1-invariants"),
        false => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.2-invariants"),
        false => |req| exact!(req, "GFDL-1.3"),
        true => |req| exact!(req, "GFDL-1.3-invariants"),
    ]);

    check!("GFDL-1.2-invariants" => [
        false => |req| exact!(req, "GFDL-1.1"),
        false => |req| exact!(req, "GFDL-1.1-invariants"),
        false => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.2-invariants"),
        false => |req| exact!(req, "GFDL-1.3"),
        false => |req| exact!(req, "GFDL-1.3-invariants"),
    ]);

    check!("GFDL-1.2-invariants-only" => [
        false => |req| exact!(req, "GFDL-1.1"),
        false => |req| exact!(req, "GFDL-1.1-invariants"),
        false => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.2-invariants"),
        false => |req| exact!(req, "GFDL-1.3"),
        false => |req| exact!(req, "GFDL-1.3-invariants"),
    ]);

    check!("GFDL-1.3-only" => [
        false => |req| exact!(req, "GFDL-1.1"),
        false => |req| exact!(req, "GFDL-1.1-invariants"),
        false => |req| exact!(req, "GFDL-1.2"),
        false => |req| exact!(req, "GFDL-1.2-invariants"),
        true => |req| exact!(req, "GFDL-1.3"),
        false => |req| exact!(req, "GFDL-1.3-invariants"),
    ]);
}

#[test]
fn noassertion() {
    check!("NOASSERTION AND OpenSSL" => [
        true && false => |req| exact!(req, "NOASSERTION") || exact!(req, "MIT"),
        true && true => |req| exact!(req, "NOASSERTION") || exact!(req, "OpenSSL"),
    ]);
}
