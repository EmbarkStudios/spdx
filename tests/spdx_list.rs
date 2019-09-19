use spdx::{exception_id, license_id};

#[test]
fn handles_deprecation() {
    assert!(license_id("GPL-3.0-with-autoconf-exception")
        .unwrap()
        .is_deprecated());
}

#[test]
fn handles_fsf() {
    let zpl = license_id("ZPL-2.1").unwrap();
    assert!(zpl.is_fsf_free_libre() && !zpl.is_osi_approved());
}

#[test]
fn handles_osi() {
    let rscpl = license_id("RSCPL").unwrap();
    assert!(rscpl.is_osi_approved() && !rscpl.is_fsf_free_libre());
}

#[test]
fn handles_fsf_and_osi() {
    let cat = license_id("Sleepycat").unwrap();

    assert!(cat.is_fsf_free_libre() && cat.is_osi_approved());
}

#[test]
fn handles_neither() {
    let adobe = license_id("Adobe-2006").unwrap();
    assert!(!adobe.is_fsf_free_libre() && !adobe.is_osi_approved());
}

#[test]
fn handles_deprecated_fsf_and_osi() {
    let id = license_id("LGPL-2.1+").unwrap();

    assert!(id.is_deprecated() && id.is_fsf_free_libre() && id.is_osi_approved());
}

#[test]
fn handles_exception_deprecation() {
    assert!(exception_id("Nokia-Qt-exception-1.1")
        .unwrap()
        .is_deprecated());
}
