// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::{
    cmp::min,
    collections::{hash_map::Iter, HashMap, VecDeque},
};

#[derive(Clone, Debug)]
pub struct NgramSet {
    pub(crate) map: HashMap<String, u32>,
    pub(crate) n: u8,
    pub(crate) size: usize,
}

impl NgramSet {
    #[inline]
    pub fn new(n: u8) -> Self {
        Self {
            map: HashMap::new(),
            n,
            size: 0,
        }
    }

    #[inline]
    pub fn from_str(s: &str, n: u8) -> Self {
        let mut set = Self::new(n);
        set.analyze(s);
        set
    }

    pub fn analyze(&mut self, s: &str) {
        let words = s.split(' ');

        let mut deque: VecDeque<&str> = VecDeque::with_capacity(self.n as usize);
        for w in words {
            deque.push_back(w);
            if deque.len() == self.n as usize {
                let gram = {
                    let mut g = String::with_capacity(deque.iter().map(|s| s.len()).sum::<usize>() + self.n as usize - 1);

                    for (i, s) in deque.iter().enumerate() {
                        if i > 0 {
                            g.push(' ');
                        }

                        g.push_str(s);
                    }

                    g
                };

                self.add_gram(gram);
                deque.pop_front();
            }
        }
    }

    #[inline]
    fn add_gram(&mut self, gram: String) {
        let n = self.map.entry(gram).or_insert(0);
        *n += 1;
        self.size += 1;
    }

    #[inline]
    pub fn get(&self, gram: &str) -> u32 {
        if let Some(count) = self.map.get(gram) {
            *count
        } else {
            0
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn dice(&self, other: &Self) -> f32 {
        // no sense comparing sets of different sizes
        if other.n != self.n {
            return 0f32;
        }

        // there's obviously no match if either are empty strings;
        // if we don't check here we could end up with NaN below
        // when both are empty
        if self.is_empty() || other.is_empty() {
            return 0f32;
        }

        // choose the smaller map to iterate
        let (x, y) = if self.len() < other.len() {
            (self, other)
        } else {
            (other, self)
        };

        let mut matches = 0;
        for (gram, count) in x {
            matches += min(*count, y.get(gram));
        }

        (2.0 * matches as f32) / ((self.len() + other.len()) as f32)
    }
}

impl PartialEq for NgramSet {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.size == other.size && self.map == other.map
    }
}

impl<'a> IntoIterator for &'a NgramSet {
    type Item = (&'a String, &'a u32);
    type IntoIter = Iter<'a, String, u32>;

    fn into_iter(self) -> Self::IntoIter {
        self.map.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // this is a pretty banal test, but it's a starting point :P
    #[test]
    fn can_construct() {
        let set = NgramSet::new(2);
        assert_eq!(set.size, 0);
        assert_eq!(set.n, 2);
    }

    #[test]
    fn no_nan() {
        let a = NgramSet::from_str("", 2);
        let b = NgramSet::from_str("", 2);

        let score = a.dice(&b);

        assert!(!score.is_nan());
    }

    #[test]
    fn same_size() {
        let a = NgramSet::from_str("", 2);
        let b = NgramSet::from_str("", 3);

        let score = a.dice(&b);

        assert_eq!(0f32, score);
    }

    #[test]
    fn identical() {
        let a = NgramSet::from_str("one two three apple banana", 2);
        let b = NgramSet::from_str("one two three apple banana", 2);

        let score = a.dice(&b);

        assert_eq!(1f32, score);
    }
}
