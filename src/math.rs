/// math helpers for bloom filter sizing
/// optimal number of bits `m` for capacity `n` and false positive rate `p`.
/// formula: m = - (n * ln p) / (ln 2)^2
pub fn optimal_m(n: usize, p: f64) -> usize {
    assert!(n > 0, "n must be > 0");
    assert!(p > 0.0 && p < 1.0, "p must be in (0,1)");
    let ln2_sq = std::f64::consts::LN_2.powi(2);
    ((-(n as f64) * p.ln()) / ln2_sq).ceil() as usize
}

/// optimal number of hash functions `k` for `m` bits and `n` items:
/// k = (m/n) * ln 2
pub fn optimal_k(m: usize, n: usize) -> u32 {
    assert!(m > 0 && n > 0);
    (((m as f64 / n as f64) * std::f64::consts::LN_2).round() as u32).max(1)
}
