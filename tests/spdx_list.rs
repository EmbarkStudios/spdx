use spdx::{exception_id, license_id};

#[test]
fn handles_deprecation() {
    let id = license_id("GPL-3.0-with-autoconf-exception").unwrap();
    assert!(id.is_deprecated());
    assert_eq!(
        id.full_name,
        "GNU General Public License v3.0 w/Autoconf exception"
    );
}

#[test]
fn handles_fsf() {
    let zpl = license_id("ZPL-2.1").unwrap();
    assert!(zpl.is_fsf_free_libre() && !zpl.is_osi_approved());
    assert_eq!(zpl.full_name, "Zope Public License 2.1");
}

#[test]
fn handles_osi() {
    let rscpl = license_id("RSCPL").unwrap();
    assert!(rscpl.is_osi_approved() && !rscpl.is_fsf_free_libre());
    assert_eq!(rscpl.full_name, "Ricoh Source Code Public License");
}

#[test]
fn handles_fsf_and_osi() {
    let cat = license_id("Sleepycat").unwrap();
    assert!(cat.is_fsf_free_libre() && cat.is_osi_approved());
    assert_eq!(cat.full_name, "Sleepycat License");
}

#[test]
fn handles_neither() {
    let adobe = license_id("Adobe-2006").unwrap();
    assert!(!adobe.is_fsf_free_libre() && !adobe.is_osi_approved());
    assert_eq!(
        adobe.full_name,
        "Adobe Systems Incorporated Source Code License Agreement"
    );
}

#[test]
fn handles_deprecated_fsf_and_osi() {
    let id = license_id("LGPL-2.1+").unwrap();
    assert!(id.is_deprecated() && id.is_fsf_free_libre() && id.is_osi_approved());

    // This is a special case, we always remove + when doing the search, but in
    // this case the + is actually a part of the name, which is why it has been deprecated,
    // but it's fine because LGPL-2.1 is also deprecated :p
    assert_eq!(id.full_name, "GNU Lesser General Public License v2.1 only");
}

#[test]
fn handles_exception_deprecation() {
    assert!(exception_id("Nokia-Qt-exception-1.1")
        .unwrap()
        .is_deprecated());
}

#[test]
fn handles_copyleft() {
    let gpl = license_id("GPL-3.0-or-later").unwrap();
    assert!(gpl.is_copyleft());
    assert_eq!(gpl.full_name, "GNU General Public License v3.0 or later");
}
