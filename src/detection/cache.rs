// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use crate::detection::{LicenseEntry, Store, license::TextData, ngram::NgramSet};
use std::io;

const CACHE_VERSION: &str = "spdx-crate-01";

#[derive(Debug)]
pub enum CacheError {
    Io(io::Error),
    InvalidVersion {
        actual: String,
        expected: &'static str,
    },
    Proto(ProtoError),
}

impl std::fmt::Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(io) => write!(f, "{io}"),
            Self::Proto(p) => write!(f, "{p}"),
            Self::InvalidVersion { actual, expected } => {
                write!(f, "expected version {expected}, but got version {actual}")
            }
        }
    }
}

impl std::error::Error for CacheError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(io) => Some(io),
            Self::Proto(p) => Some(p),
            Self::InvalidVersion { .. } => None,
        }
    }
}

impl From<BinErr> for CacheError {
    fn from(b: BinErr) -> Self {
        match b {
            BinErr::Io(i) => Self::Io(i),
            BinErr::Proto(p) => Self::Proto(p),
        }
    }
}

impl From<io::Error> for CacheError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl Store {
    /// Create a store from a cache file.
    ///
    /// This method is highly useful for quickly loading a cache, as creating
    /// one from text data is rather slow. This method can typically load
    /// the full SPDX set from disk in < 100ms.
    ///
    /// The cache contains a simple version header that ensure that the cache
    /// is loadable
    pub fn from_cache<R>(mut readable: R) -> Result<Self, CacheError>
    where
        R: io::Read + Sized,
    {
        let mut header = [0u8; 13];
        readable.read_exact(&mut header)?;

        if header != CACHE_VERSION.as_bytes() {
            return Err(CacheError::InvalidVersion {
                actual: String::from_utf8_lossy(&header).into_owned(),
                expected: CACHE_VERSION,
            });
        }

        let mut dec = zstd::Decoder::new(readable)?;
        Ok(Self::bread(&mut dec)?)
    }

    /// Serialize the current store.
    pub fn to_cache<W>(&self, mut writable: W) -> Result<(), CacheError>
    where
        W: io::Write + Sized,
    {
        writable.write_all(CACHE_VERSION.as_bytes())?;

        let mut enc = zstd::Encoder::new(writable, 21)?;
        self.bwrite(&mut enc)?;
        enc.finish()?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum ProtoError {
    TooLong(usize),
    Utf8(std::string::FromUtf8Error),
}

impl std::fmt::Display for ProtoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLong(tl) => write!(f, "{tl:016x} is too large to fit in a u16"),
            Self::Utf8(u) => write!(f, "{u}"),
        }
    }
}

impl std::error::Error for ProtoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        if let Self::Utf8(u) = self {
            Some(u)
        } else {
            None
        }
    }
}

enum BinErr {
    Io(io::Error),
    Proto(ProtoError),
}

impl From<io::Error> for BinErr {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ProtoError> for BinErr {
    fn from(e: ProtoError) -> Self {
        Self::Proto(e)
    }
}

#[inline]
fn write_u16<W>(u: usize, w: &mut W) -> Result<(), BinErr>
where
    W: io::Write + Sized,
{
    let u: u16 = u.try_into().map_err(|_e| ProtoError::TooLong(u))?;
    w.write_all(&u.to_le_bytes()).map_err(BinErr::Io)
}

#[inline]
fn read_u16<R>(r: &mut R) -> Result<usize, BinErr>
where
    R: io::Read + Sized,
{
    let mut u = [0u8; 2];
    r.read_exact(&mut u)?;
    Ok(u16::from_le_bytes(u) as usize)
}

#[inline]
fn write_u64<W>(u: usize, w: &mut W) -> Result<(), BinErr>
where
    W: io::Write + Sized,
{
    w.write_all(&(u as u64).to_le_bytes()).map_err(BinErr::Io)
}

#[inline]
fn read_u64<R>(r: &mut R) -> Result<usize, BinErr>
where
    R: io::Read + Sized,
{
    let mut b = [0u8; 8];
    r.read_exact(&mut b)?;
    Ok(u64::from_le_bytes(b) as usize)
}

impl Bin for String {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized,
    {
        write_u16(self.len(), w)?;
        w.write_all(self.as_bytes()).map_err(BinErr::Io)
    }

    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized,
    {
        let mut len = read_u16(r)?;
        let mut pos = 0;
        let mut s = vec![0; len];

        while len > 0 {
            let read = r.read(&mut s[pos..])?;
            pos += read;
            len -= read;
        }

        Ok(String::from_utf8(s).map_err(ProtoError::Utf8)?)
    }
}

#[inline]
fn write_vec<W, B>(v: &[B], w: &mut W) -> Result<(), BinErr>
where
    W: io::Write + Sized,
    B: Bin,
{
    write_u16(v.len(), w)?;

    for b in v {
        b.bwrite(w)?;
    }

    Ok(())
}

#[inline]
fn read_vec<R, B>(r: &mut R) -> Result<Vec<B>, BinErr>
where
    R: io::Read + Sized,
    B: Bin,
{
    let len = read_u16(r)?;

    let mut v = Vec::with_capacity(len);

    for _ in 0..len {
        v.push(B::bread(r)?);
    }

    Ok(v)
}

trait Bin: Sized {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized;
    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized;
}

impl Bin for Store {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized,
    {
        write_u16(self.licenses.len(), w)?;

        for (k, v) in &self.licenses {
            k.bwrite(w)?;
            v.bwrite(w)?;
        }

        Ok(())
    }

    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized,
    {
        let map_count = read_u16(r)?;

        let mut licenses = std::collections::HashMap::new();

        for _ in 0..map_count {
            let key = String::bread(r)?;
            let value = LicenseEntry::bread(r)?;

            licenses.insert(key, value);
        }

        Ok(Self { licenses })
    }
}

impl Bin for LicenseEntry {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized,
    {
        self.original.bwrite(w)?;
        write_vec(&self.aliases, w)?;
        write_vec(&self.headers, w)?;
        write_vec(&self.alternates, w)?;

        Ok(())
    }

    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized,
    {
        Ok(Self {
            original: TextData::bread(r)?,
            aliases: read_vec(r)?,
            headers: read_vec(r)?,
            alternates: read_vec(r)?,
        })
    }
}

impl Bin for TextData {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized,
    {
        self.match_data.bwrite(w)?;
        write_u64(self.lines_view.0, w)?;
        write_u64(self.lines_view.1, w)?;
        write_vec(&self.lines_normalized, w)?;
        self.text_processed.bwrite(w)?;

        Ok(())
    }

    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized,
    {
        Ok(Self {
            match_data: NgramSet::bread(r)?,
            lines_view: (read_u64(r)?, read_u64(r)?),
            lines_normalized: read_vec(r)?,
            text_processed: String::bread(r)?,
        })
    }
}

impl Bin for u32 {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized,
    {
        w.write_all(&self.to_le_bytes()).map_err(BinErr::Io)
    }

    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized,
    {
        let mut b = [0; 4];
        r.read_exact(&mut b)?;
        Ok(u32::from_le_bytes(b))
    }
}

impl Bin for NgramSet {
    fn bwrite<W>(&self, w: &mut W) -> Result<(), BinErr>
    where
        W: io::Write + Sized,
    {
        write_u16(self.map.len(), w)?;
        for (k, v) in &self.map {
            k.bwrite(w)?;
            v.bwrite(w)?;
        }
        w.write_all(&[self.n])?;
        write_u64(self.size, w)?;

        Ok(())
    }

    fn bread<R>(r: &mut R) -> Result<Self, BinErr>
    where
        R: io::Read + Sized,
    {
        let map_len = read_u16(r)?;
        let mut map = std::collections::HashMap::new();
        for _ in 0..map_len {
            let k = String::bread(r)?;
            let v = u32::bread(r)?;

            map.insert(k, v);
        }
        let mut n = [0; 1];
        r.read_exact(&mut n)?;
        let size = read_u64(r)?;

        Ok(Self { map, n: n[0], size })
    }
}
