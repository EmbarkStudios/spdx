#![allow(clippy::nonminimal_bool, clippy::eq_op, clippy::cognitive_complexity)]

use spdx::LicenseItem;

macro_rules! exact {
    ($req:expr, $e:expr) => {
        spdx::Licensee::parse_mode($e, spdx::ParseMode::LAX)
            .unwrap()
            .satisfies($req)
    };
}

macro_rules! check {
    ($le:expr => [$($logical_expr:expr => $is_allowed:expr),+$(,)?]) => {
        check_mode!(spdx::ParseMode::STRICT, $le => [$($logical_expr => $is_allowed),+])
    };
}

macro_rules! check_lax {
    ($le:expr => [$($logical_expr:expr => $is_allowed:expr),+$(,)?]) => {
        check_mode!(spdx::ParseMode::LAX, $le => [$($logical_expr => $is_allowed),+])
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
        true && (true || false) => |req| exact!(req, "MIT") || exact!(req, "LGPL-3.0-or-later"),
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
#[allow(clippy::blocks_in_conditions)]
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
        true => |req| exact!(req, "GPL-3.0-only"),
        true => |req| exact!(req, "GPL-3.0"),
        true => |req| exact!(req, "GPL-3.0-or-later"),
        //true => |req| exact!(req, "GPL-4.0"),
    ]);
}

#[test]
fn gpl_or_later_plus_strict() {
    spdx::Expression::parse("GPL-2.0+").unwrap_err();
}

#[test]
fn gpl_or_later_plus_lax() {
    spdx::Expression::parse_mode("GPL-2.0+", spdx::ParseMode::LAX).unwrap();
}

#[test]
fn gpl_pedantic() {
    // | Licensee | GPL-1.0-only  | GPL-1.0-or-later | GPL-2.0-only | GPL-2.0-or-later | GPL-3.0-only | GPL-3.0-or-later |
    // | ----------------- | -- | -- | -- | -- | -- | -- |
    // | GPL-1.0-only      | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
    // | GPL-1.0-or-later  | ✅ | ✅ | ❌ | ❌ | ❌ | ❌ |
    // | GPL-2.0-only      | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
    // | GPL-2.0-or-later  | ❌ | ✅ | ✅ | ✅ | ❌ | ❌ |
    // | GPL-3.0-only      | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |
    // | GPL-3.0-or-later  | ❌ | ✅ | ❌ | ✅ | ✅ | ✅ |

    const ONE_ONLY: &str = "GPL-1.0-only";
    const ONE_LATER: &str = "GPL-1.0-or-later";
    const TWO_ONLY: &str = "GPL-2.0-only";
    const TWO_LATER: &str = "GPL-2.0-or-later";
    const THREE_ONLY: &str = "GPL-3.0-only";
    const THREE_LATER: &str = "GPL-3.0-or-later";

    let table = [
        (
            ONE_ONLY,
            [
                (ONE_ONLY, true),
                (ONE_LATER, true),
                (TWO_ONLY, false),
                (TWO_LATER, false),
                (THREE_ONLY, false),
                (THREE_LATER, false),
            ],
        ),
        (
            ONE_LATER,
            [
                (ONE_ONLY, true),
                (ONE_LATER, true),
                (TWO_ONLY, false),
                (TWO_LATER, false),
                (THREE_ONLY, false),
                (THREE_LATER, false),
            ],
        ),
        (
            TWO_ONLY,
            [
                (ONE_ONLY, false),
                (ONE_LATER, true),
                (TWO_ONLY, true),
                (TWO_LATER, true),
                (THREE_ONLY, false),
                (THREE_LATER, false),
            ],
        ),
        (
            TWO_LATER,
            [
                (ONE_ONLY, false),
                (ONE_LATER, true),
                (TWO_ONLY, true),
                (TWO_LATER, true),
                (THREE_ONLY, false),
                (THREE_LATER, false),
            ],
        ),
        (
            THREE_ONLY,
            [
                (ONE_ONLY, false),
                (ONE_LATER, true),
                (TWO_ONLY, false),
                (TWO_LATER, true),
                (THREE_ONLY, true),
                (THREE_LATER, true),
            ],
        ),
        (
            THREE_LATER,
            [
                (ONE_ONLY, false),
                (ONE_LATER, true),
                (TWO_ONLY, false),
                (TWO_LATER, true),
                (THREE_ONLY, true),
                (THREE_LATER, true),
            ],
        ),
    ];

    for (licensee, items) in table {
        let lic = spdx::Licensee::parse(licensee).unwrap();

        for (req, passes) in items {
            let req = spdx::LicenseReq {
                license: spdx::LicenseItem::Spdx {
                    id: spdx::license_id(req).unwrap(),
                    or_later: false,
                },
                exception: None,
            };

            assert_eq!(lic.satisfies(&req), passes);
        }
    }
}

#[test]
fn gfdl() {
    check!("GFDL-1.2-or-later" => [
        false => |req| exact!(req, "GFDL-1.1"),
        true => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.3"),
        false => |req| exact!(req, "GFDL-1.1-or-later"),
        true => |req| exact!(req, "GFDL-1.2-or-later"),
        true => |req| exact!(req, "GFDL-1.3-or-later"),
    ]);

    check!("GFDL-1.2-invariants-or-later" => [
        false => |req| exact!(req, "GFDL-1.1"),
        false => |req| exact!(req, "GFDL-1.1-invariants"),
        false => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.2-invariants-or-later"),
        false => |req| exact!(req, "GFDL-1.3"),
        true => |req| exact!(req, "GFDL-1.3-invariants-only"),
    ]);

    check_lax!("GFDL-1.1-invariants+" => [
        false => |req| exact!(req, "GFDL-1.1"),
        true => |req| exact!(req, "GFDL-1.1-invariants-or-later"),
        false => |req| exact!(req, "GFDL-1.2"),
        true => |req| exact!(req, "GFDL-1.2-invariants-or-later"),
        false => |req| exact!(req, "GFDL-1.3"),
        true => |req| exact!(req, "GFDL-1.3-invariants-or-later"),
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
fn bsd() {
    check!("BSD-1-Clause+" => [
        true => |req| exact!(req, "BSD-1-Clause"),
        true => |req| exact!(req, "BSD-2-Clause"),
        true => |req| exact!(req, "BSD-3-Clause"),
        false => |req| exact!(req, "BSD-2-Clause-Darwin"),
        false => |req| exact!(req, "BSD-2-Clause-pkgconf-disclaimer"),
        false => |req| exact!(req, "BSD-3-Clause-flex"),
    ]);
}

#[test]
fn noassertion() {
    check!("NOASSERTION AND OpenSSL" => [
        true && false => |req| exact!(req, "NOASSERTION") || exact!(req, "MIT"),
        true && true => |req| exact!(req, "NOASSERTION") || exact!(req, "OpenSSL"),
    ]);
}

#[test]
fn many_ands() {
    check!("ISC AND OpenSSL AND MIT" => [
        false && true && true => |req| exact!(req, "OpenSSL") || exact!(req, "MIT"),
        true && false && true => |req| exact!(req, "ISC") || exact!(req, "MIT"),
        true && true && false => |req| exact!(req, "ISC") || exact!(req, "OpenSSL"),
        true && true && true => |req| exact!(req, "ISC") || exact!(req, "OpenSSL") || exact!(req, "MIT"),
    ]);
}

#[test]
fn minimizes_vanilla() {
    let expr = spdx::Expression::parse("Apache-2.0 OR MIT").unwrap();

    let accepted = [
        &spdx::Licensee::parse("Apache-2.0").unwrap(),
        &spdx::Licensee::parse("MIT").unwrap(),
    ];

    // We accept both Apache-2.0 and MIT, but since we only need one of them and
    // Apache-2.0 is higher priority, it gets chosen
    assert_eq!(
        expr.minimized_requirements(accepted).unwrap(),
        vec![spdx::LicenseReq {
            license: LicenseItem::Spdx {
                id: spdx::license_id("Apache-2.0").unwrap(),
                or_later: false,
            },
            exception: None,
        }]
    );

    let accepted = [
        &spdx::Licensee::parse("MIT").unwrap(),
        &spdx::Licensee::parse("Apache-2.0").unwrap(),
    ];

    assert_eq!(
        expr.minimized_requirements(accepted).unwrap(),
        vec![spdx::LicenseReq {
            license: LicenseItem::Spdx {
                id: spdx::license_id("MIT").unwrap(),
                or_later: false,
            },
            exception: None,
        }]
    );
}

#[test]
fn handles_unminimizable() {
    let expr = spdx::Expression::parse("ISC AND OpenSSL AND MIT").unwrap();
    let accepted = [
        &spdx::Licensee::parse("Apache-2.0").unwrap(),
        &spdx::Licensee::parse("ISC").unwrap(),
        &spdx::Licensee::parse("OpenSSL").unwrap(),
        &spdx::Licensee::parse("MIT").unwrap(),
    ];

    assert_eq!(
        expr.minimized_requirements(accepted).unwrap(),
        vec![
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("ISC").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("OpenSSL").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("MIT").unwrap(),
                    or_later: false,
                },
                exception: None,
            }
        ]
    );
}

#[test]
fn handles_complicated() {
    let expr = spdx::Expression::parse("ISC AND OpenSSL AND (MIT OR Apache-2.0)").unwrap();
    let accepted = [
        &spdx::Licensee::parse("Apache-2.0").unwrap(),
        &spdx::Licensee::parse("ISC").unwrap(),
        &spdx::Licensee::parse("OpenSSL").unwrap(),
        &spdx::Licensee::parse("MIT").unwrap(),
    ];

    assert_eq!(
        expr.minimized_requirements(accepted).unwrap(),
        vec![
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("Apache-2.0").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("ISC").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("OpenSSL").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
        ]
    );

    let accepted = [
        &spdx::Licensee::parse("MIT").unwrap(),
        &spdx::Licensee::parse("Apache-2.0").unwrap(),
        &spdx::Licensee::parse("ISC").unwrap(),
        &spdx::Licensee::parse("OpenSSL").unwrap(),
    ];

    assert_eq!(
        expr.minimized_requirements(accepted).unwrap(),
        vec![
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("MIT").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("ISC").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
            spdx::LicenseReq {
                license: LicenseItem::Spdx {
                    id: spdx::license_id("OpenSSL").unwrap(),
                    or_later: false,
                },
                exception: None,
            },
        ]
    );
}

#[test]
fn unsatisfied_minimize() {
    let unsatisfied = spdx::Expression::parse("Apache-2.0 OR MIT").unwrap();
    let accepted = [&spdx::Licensee::parse("BSD-3-Clause").unwrap()];

    assert_eq!(
        unsatisfied.minimized_requirements(accepted).unwrap_err(),
        spdx::expression::MinimizeError::RequirementsUnmet,
    );
}

#[test]
fn too_many_to_minimize() {
    let mut ridiculous = String::new();
    let mut ohno = Vec::new();
    for (lic, _, flags) in spdx::identifiers::LICENSES {
        if (flags & spdx::identifiers::IS_GNU) == 0 {
            ridiculous.push_str(lic);
            ridiculous.push_str(" AND ");

            ohno.push(spdx::Licensee::parse_mode(lic, spdx::ParseMode::LAX).unwrap());
        }

        if ohno.len() >= 65 {
            break;
        }
    }

    ridiculous.truncate(ridiculous.len() - 5);

    let ridiculous = spdx::Expression::parse_mode(&ridiculous, spdx::ParseMode::LAX).unwrap();

    assert_eq!(
        ridiculous.minimized_requirements(ohno.iter()).unwrap_err(),
        spdx::expression::MinimizeError::TooManyRequirements(65)
    );
}
