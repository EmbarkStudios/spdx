#![allow(clippy::nonminimal_bool, clippy::eq_op, clippy::cognitive_complexity)]

use spdx::LicenseItem;

macro_rules! exact {
    ($req:expr, $e:expr) => {
        spdx::Licensee::parse($e).unwrap().satisfies($req)
    };
}

macro_rules! check {
    ($le:expr => [$($logical_expr:expr => $is_allowed:expr),+$(,)?]) => {
        let validated = spdx::ValidExpression::parse($le).unwrap();

        $(
            // Evaluate the logical expression to determine if we are
            // expecting an Ok or Err
            let expected = $logical_expr;

            match validated.evaluate_with_failures($is_allowed) {
                Ok(_) => assert!(expected, stringify!($logical_expr)),
                Err(f) => assert!(!expected, "{} {:?}", stringify!($logical_expr), f),
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
    check!("(MIT AND (LGPL-2.1+ OR BSD-3-Clause))" => [
        false && (false || true) => |req| exact!(req, "MIT"),
        false && (false || false) => |req| exact!(req, "Apache-2.0"),
        true && (false || false) => |req| exact!(req, "MIT"),
        true && (false || true) => |req| exact!(req, "MIT") || exact!(req, "BSD-3-Clause"),
        true && (true || false) => |req| exact!(req, "MIT") || exact!(req, "LGPL-2.1"),
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
            if let LicenseItem::SPDX { id, .. } = req.license {
                return id.is_osi_approved() || id.is_fsf_free_libre();
            }

            false
        },
        false || true && false => |req| {
            if let LicenseItem::SPDX { id, .. } = req.license {
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
