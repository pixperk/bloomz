#[cfg(feature = "serde")] use serde::{Serialize, Deserialize};

/// Compact fixed-size bit set storing bits in a Vec<u64>.
///
/// Used internally by the Bloom filter but can be reused for other
/// bit-indexable purposes. Indexing is zero-based.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BitSet{
    words :  Vec<u64>,
    bits : usize,
}

impl BitSet{
    /// Create a new BitSet able to hold `bits` bits, all initialized to 0.
    pub fn new(bits : usize) -> Self {
        let words = bits.div_ceil(64);
        Self { words: 
            vec![0u64; words], bits }
    }

    /// Construct directly from a words vector (length must match bits.div_ceil(64)).
    #[allow(dead_code)]
    pub(crate) fn from_words(bits: usize, words: Vec<u64>) -> Self {
        let expected = bits.div_ceil(64);
        assert_eq!(words.len(), expected, "words length mismatch for bits");
        Self { words, bits }
    }

    /// Set (turn on) the bit at global index `idx`.
    /// Panics if `idx >= self.bits`.
    pub fn set(&mut self, idx : usize){
        let word = idx/64;
        let bit = idx % 64;
        self.words[word] |= 1u64 << bit;
    }

    /// Get the value of the bit at index `idx` (true if set).
    /// Panics if `idx >= self.bits`.
    pub fn get(&self, idx : usize) -> bool {
        let word = idx/64;
        let bit = idx % 64;
        (self.words[word] >> bit) & 1u64 == 1u64
    }

    /// In-place bitwise OR with another BitSet (sizes must match).
    pub fn or_with(&mut self, other : &BitSet){
        assert_eq!(self.words.len(), other.words.len(), "bitset size mismatch");
         for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
         }
    }

    /// In-place bitwise AND with another BitSet (sizes must match).
    pub fn and_with(&mut self, other : &BitSet){
        assert_eq!(self.words.len(), other.words.len(), "bitset size mismatch");
         for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
         }
    }

    /// Clear all bits (set to 0).
    pub fn clear(&mut self) {
        self.words.fill(0);
    }

    /// Total number of addressable bits.
    pub fn len_bits(&self) -> usize {
        self.bits
    }

    /// Immutable slice of the underlying 64-bit words (little-endian bit order inside each word).
    pub fn words_slice(&self) -> &[u64] {
        &self.words
    }

    /// Mutable slice of the underlying 64-bit words (use with care).
    pub fn words_mut(&mut self) -> &mut [u64] {
        &mut self.words
    }
}
