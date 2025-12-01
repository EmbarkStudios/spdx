#![cfg(feature = "detection")]

#[cfg(feature = "detection-inline-cache")]
#[test]
fn reads_inline_cache() {
    let store = spdx::detection::Store::load_inline().expect("failed to load cache");

    let mut set = std::collections::BTreeSet::new();

    for (k, v) in store.iter() {
        set.insert(k.as_str());

        for alias in &v.aliases {
            set.insert(alias.as_str());
        }
    }

    // We manually add the NOASSERTION "fake" license id since it's not part of
    // SPDX, but might be in the future https://github.com/spdx/spdx-spec/issues/50
    // so that should be the only license that isn't present in the store
    for lic in spdx::identifiers::LICENSES {
        if lic.name != "NOASSERTION" {
            assert!(set.contains(lic.name), "failed to find expected license {} in inline cache store", lic.name);
        }
    }
}