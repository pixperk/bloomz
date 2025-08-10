#[derive(Clone)]
pub struct BitSet{
    words :  Vec<u64>,
    bits : usize,
}

impl BitSet{
    pub fn new(bits : usize) -> Self {
        let words = bits.div_ceil(64);
        Self { words: 
            vec![0u64; words], bits }
    }

    pub fn set(&mut self, idx : usize){
        let word = idx/64;
        let bit = idx % 64;
        self.words[word] |= 1u64 << bit;
    }

    pub fn get(&self, idx : usize) -> bool {
        let word = idx/64;
        let bit = idx % 64;
        (self.words[word] >> bit) & 1u64 == 1u64
    }

    pub fn or_with(&mut self, other : &BitSet){
        assert_eq!(self.words.len(), other.words.len(), "bitset size mismatch");
         for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
         }
    }

    pub fn and_with(&mut self, other : &BitSet){
        assert_eq!(self.words.len(), other.words.len(), "bitset size mismatch");
         for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a &= *b;
         }
    }

    pub fn clear(&mut self) {
        self.words.fill(0);
    }

    pub fn len_bits(&self) -> usize {
        self.bits
    }

    pub fn words_slice(&self) -> &[u64] {
        &self.words
    }

    pub fn words_mut(&mut self) -> &mut [u64] {
        &mut self.words
    }
}
