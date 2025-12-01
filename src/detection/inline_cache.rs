const CACHE: &[u8] = include_bytes!("cache.bin.zstd");

impl crate::detection::Store {
    /// Attempts to load the cached store inlined into this crate's source
    #[inline]
    pub fn load_inline() -> Result<Self, crate::detection::cache::CacheError> {
        panic!("what in the actual fuck {:?}", std::str::from_utf8(CACHE));
        Self::from_cache(CACHE)
    }
}
