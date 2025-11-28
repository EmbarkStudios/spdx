// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! This module is basically an inling of [askalono](https://github.com/jpeddicord/askalono)
//! 
//! Askalono is not really maintained and also depends on other unmaintained
//! crates, since this crate is used by both cargo-deny and cargo-about in
//! conjunction with askalono for checking licenses, I'm pulling it directly into
//! this crate just to avoid all of the external dependencies

use std::collections::HashMap;

#[cfg(feature = "detection-cache")]
mod cache;
#[cfg(feature = "detection-inline-cache")]
mod inline_cache;
mod detect;
mod license;
pub use license::{LicenseType, TextData};
mod ngram;
mod preproc;

pub struct LicenseEntry {
    pub original: TextData,
    pub aliases: Vec<String>,
    pub headers: Vec<TextData>,
    pub alternates: Vec<TextData>,
}

impl LicenseEntry {
    pub fn new(original: TextData) -> Self {
        Self {
            original,
            aliases: Vec::new(),
            alternates: Vec::new(),
            headers: Vec::new(),
        }
    }
}

/// A representation of a collection of known licenses.
///
/// This struct is generally what you want to start with if you're looking to
/// match text against a database of licenses. Load a cache from disk using
/// `from_cache`, then use the `analyze` function to determine what a text most
/// closely matches.
#[derive(Default)]
pub struct Store {
    pub(crate) licenses: HashMap<String, LicenseEntry>,
}

impl Store {
    /// Create a new `Store`.
    ///
    /// More often, you probably want to use `from_cache` instead of creating
    /// an empty store.
    pub fn new() -> Self {
        Self {
            licenses: HashMap::new(),
        }
    }

    /// Get the number of licenses in the store.
    ///
    /// This only counts licenses by name -- headers, aliases, and alternates
    /// aren't included in the count.
    #[inline]
    pub fn len(&self) -> usize {
        self.licenses.len()
    }

    /// Check if the store is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.licenses.is_empty()
    }

    /// Get all licenses by name via iterator.
    #[inline]
    pub fn licenses(&self) -> impl Iterator<Item = &String> {
        self.licenses.keys()
    }

    /// Get a license's standard `TextData` by name.
    #[inline]
    pub fn get_original(&self, name: &str) -> Option<&TextData> {
        self.licenses.get(name).map(|le| &le.original)
    }

    /// Add a single license to the store.
    ///
    /// If the license with the given name already existed, it and all of its
    /// variants will be replaced.
    #[inline]
    pub fn add_license(&mut self, name: String, data: TextData) {
        let entry = LicenseEntry::new(data);
        self.licenses.insert(name, entry);
    }

    /// Inserts a full `LicenseEntry`
    #[inline]
    pub fn insert_entry(&mut self, name: String, entry: LicenseEntry) {
        self.licenses.insert(name, entry);
    }

    /// Add a variant (a header or alternate formatting) of a given license to
    /// the store.
    ///
    /// The license must already exist. This function cannot be used to replace
    /// the original/canonical text of the license.
    #[inline]
    pub fn add_variant(
        &mut self,
        name: &str,
        variant: LicenseType,
        data: TextData,
    ) -> Result<(), StoreError> {
        let entry = self
            .licenses
            .get_mut(name)
            .ok_or(StoreError::UnknownLicense)?;
        
        match variant {
            LicenseType::Alternate => {
                entry.alternates.push(data);
            }
            LicenseType::Header => {
                entry.headers.push(data);
            }
            LicenseType::Original => {
                return Err(StoreError::OriginalInvalidForVariant);
            }
        }

        Ok(())
    }

    /// Get the list of aliases for a given license.
    #[inline]
    pub fn aliases(&self, name: &str) -> Option<&Vec<String>> {
        self
            .licenses
            .get(name).map(|le| &le.aliases)
    }

    /// Set the list of aliases for a given license.
    #[inline]
    pub fn set_aliases(&mut self, name: &str, aliases: Vec<String>) -> Result<(), StoreError> {
        let entry = self
            .licenses
            .get_mut(name)
            .ok_or(StoreError::UnknownLicense)?;
        entry.aliases = aliases;
        Ok(())
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum StoreError {
    /// The license name was not in the Store
    UnknownLicense,
    /// Attempted to call `Store::add_variant` with `LicenseType::Original`
    OriginalInvalidForVariant,
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownLicense => f.write_str("specified license did not exist in the store"),
            Self::OriginalInvalidForVariant => f.write_str("attempted to add an original license text as a variant"),
        }
    }
}

impl std::error::Error for StoreError {}