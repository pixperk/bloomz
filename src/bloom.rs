use core::hash::{BuildHasher, Hash};
use core::marker::PhantomData;

use std::fmt;

use crate::{bitset::BitSet, hashing, math};
/// bloom filter with configurable BuildHasher `S`.
///
/// `S` defaults to `std::collections::hash_map::RandomState` which uses SipHash (safe).
#[derive(Clone)]
pub struct BloomFilter<S = std::collections::hash_map::RandomState> {
    bits: BitSet,
    m: usize, //number of bits
    k: u32,   //hash funcs
    items: usize,
    hasher_builder: S,
    _marker: PhantomData<S>,
}

impl<S> fmt::Debug for BloomFilter<S>
where
    S: BuildHasher + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BloomFilter")
            .field("m(bits)", &self.m)
            .field("k", &self.k)
            .field("items", &self.items)
            .finish()
    }
}

impl BloomFilter<std::collections::hash_map::RandomState> {
    /// convenience constructor using default hasher builder.
    pub fn new(m: usize, k: u32) -> Self {
        Self::with_hasher(m, k, std::collections::hash_map::RandomState::new())
    }

    /// convenience constructor from capacity and false-positive rate with default hasher.
    pub fn new_for_capacity(n: usize, p: f64) -> Self {
        let m = math::optimal_m(n, p);
        let k = math::optimal_k(m, n);
        Self::with_hasher(m, k, std::collections::hash_map::RandomState::new())
    }
}

impl<S> BloomFilter<S>
where
    S: BuildHasher + Clone,
{
    /// create with explicit hasher builder (eg. ahash::AHasherBuilder or RandomState)
    pub fn with_hasher(m: usize, k: u32, hasher_builder: S) -> Self {
        assert!(m > 0 && k > 0);
        Self {
            bits: BitSet::new(m),
            m,
            k,
            items: 0,
            hasher_builder,
            _marker: PhantomData,
        }
    }

    ///insert item
    pub fn insert<T : Hash>(&mut self, item : &T){
        let (h1, h2) = hashing::hash2(&self.hasher_builder, item);
        for i in 0..self.k{
            let combined = h1.wrapping_add((i as u64).wrapping_mul(h2));
            let idx = (combined % (self.m as u64)) as usize;
            self.bits.set(idx);
        }
        self.items = self.items.saturating_add(1);
    }

    pub fn contains<T : Hash>(&self, item : &T) -> bool{
        let (h1, h2) = hashing::hash2(&self.hasher_builder, item);
       for i in 0..self.k {
            let combined = h1.wrapping_add((i as u64).wrapping_mul(h2));
            let idx = (combined % (self.m as u64)) as usize;
            if !self.bits.get(idx) {
                return false;
            }
        }
        true
    }

    /// union (bitwise OR). both must have same m and k.
    pub fn union_inplace(&mut self, other: &Self) {
        assert_eq!(self.m, other.m, "m mismatch for union");
        assert_eq!(self.k, other.k, "k mismatch for union");
        self.bits.or_with(&other.bits);
    }

    /// intersection (bitwise AND). both must have same m and k.
    pub fn intersect_inplace(&mut self, other: &Self) {
        assert_eq!(self.m, other.m, "m mismatch for intersection");
        assert_eq!(self.k, other.k, "k mismatch for intersection");
        self.bits.and_with(&other.bits);
    }

    /// clear all bits
    pub fn clear(&mut self) {
        self.bits.clear();
        self.items = 0;
    }

    /// approximate count of insert calls (not exact, duplicates counted)
    pub fn approximate_items(&self) -> usize {
        self.items
    }

    /// serialize to bytes: layout = words (u64 LE) + m (u64 LE) + k (u32 LE)
    pub fn to_bytes(&self) -> Vec<u8> {
        let words = self.bits.words_slice();
        let mut out = Vec::with_capacity(words.len() * 8 + 12);
        for w in words {
            out.extend_from_slice(&w.to_le_bytes());
        }
        out.extend_from_slice(&(self.m as u64).to_le_bytes());
        out.extend_from_slice(&self.k.to_le_bytes());
        out
    }

    /// deserialize (expects same layout as to_bytes)
    pub fn from_bytes_hasher(data: &[u8], hasher_builder: S) -> Option<Self> {
        if data.len() < 12 { return None; }
        let meta_offset = data.len() - 12;
        let mut m_bytes = [0u8; 8];
        m_bytes.copy_from_slice(&data[meta_offset..meta_offset+8]);
        let m = u64::from_le_bytes(m_bytes) as usize;

        let mut k_bytes = [0u8; 4];
        k_bytes.copy_from_slice(&data[meta_offset+8..meta_offset+12]);
        let k = u32::from_le_bytes(k_bytes);

        let words_expected = (m + 63) / 64;
        if meta_offset != words_expected * 8 { return None; }

        let mut words = Vec::with_capacity(words_expected);
        for i in 0..words_expected {
            let start = i * 8;
            let mut wb = [0u8; 8];
            wb.copy_from_slice(&data[start..start+8]);
            words.push(u64::from_le_bytes(wb));
        }

        let mut bitset = BitSet::new(m);
        bitset.words_mut().copy_from_slice(&words);

        Some(Self {
            bits: bitset,
            m,
            k,
            items: 0,
            hasher_builder,
            _marker: PhantomData,
        })
    }

    /// convenience wrapper using default hasher builder
    pub fn from_bytes(data: &[u8]) -> Option<Self>
    where
        std::collections::hash_map::RandomState: Clone,
        S: From<std::collections::hash_map::RandomState>,
    {
        // if S can be constructed from RandomState, build one and call from_bytes_hasher
        let rs = std::collections::hash_map::RandomState::new();
        let builder: S = rs.into();
        Self::from_bytes_hasher(data, builder)
    }
}
