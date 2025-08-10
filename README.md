# bloomz

Fast, flexible Bloom filter for Rust with pluggable hash builders.

## Features
- Customizable number of bits (m) and hash functions (k)
- Capacity / false-positive rate constructors
- Generic over `BuildHasher` (defaults to `RandomState` / SipHash)
- Union & intersection
- Binary serialization (`to_bytes` / `from_bytes[_hasher]`)
- Optional serde support (feature `serde`)

## Add to your project
```toml
[dependencies]
bloomz = { path = "./bloomz" } # or from crates.io once published
# Optional
bincode = "1"
serde_json = "1"
```

Enable serde in Cargo.toml if needed:
```toml
bloomz = { version = "*", features = ["serde"] }
```

## Quick start
```rust
use bloomz::BloomFilter;

fn main() {
    // Create from explicit m & k
    let mut bf = BloomFilter::new(10_000, 7);
    bf.insert(&"hello");
    assert!(bf.contains(&"hello"));
    assert!(!bf.contains(&"world"));

    // Create from capacity & target false positive probability p
    let mut bf2 = BloomFilter::new_for_capacity(50_000, 0.01);
    bf2.insert(&42u64);
    assert!(bf2.contains(&42u64));
}
```

## Choosing m & k automatically
```rust
let n = 100_000;          // expected inserts
let p = 0.005;            // desired false positive probability
let mut bf = BloomFilter::new_for_capacity(n, p);
```

## Using a faster (non-cryptographic) hasher
The default `RandomState` (SipHash) is DOS‑resistant but slower. For speed, pick a hasher like `ahash` (good balance of speed & quality) or `fxhash` (simple, very fast).

Add a dependency:
```toml
[dependencies]
ahash = "0.8"
# or
# fxhash = "0.2"
```

Use with `with_hasher`:
```rust
use bloomz::BloomFilter;
use ahash::RandomState as AHashState; // fast hash builder

let mut bf = BloomFilter::with_hasher(100_000, 6, AHashState::new());
for key in 0u64..10_000 { bf.insert(&key); }
assert!(bf.contains(&1234u64));
```
`fxhash` example:
```rust
use bloomz::BloomFilter;
use fxhash::FxBuildHasher; // provides BuildHasher

let mut bf = BloomFilter::with_hasher(50_000, 5, FxBuildHasher::default());
```
Pick k empirically (5–8 typical) or rely on `new_for_capacity` + swap hasher after computing parameters.

## Serialization (custom binary)
```rust
let mut bf = BloomFilter::new_for_capacity(5_000, 0.01);
for i in 0..1000u32 { bf.insert(&i); }
let bytes = bf.to_bytes();
// Persist bytes ...
let restored = BloomFilter::from_bytes(&bytes).unwrap();
assert!(restored.contains(&123u32));
```
If you used a custom hasher, use `from_bytes_hasher(bytes, hasher_builder)` with the same `BuildHasher` type.

## Serde integration (optional feature)
Enable the feature `serde` and (optionally) use formats like JSON, CBOR, or bincode.

### JSON example
```rust
use bloomz::BloomFilter;
#[cfg(feature = "serde")] {
    let mut bf = BloomFilter::new_for_capacity(10_000, 0.01);
    bf.insert(&"alpha");
    bf.insert(&"beta");

    let json = serde_json::to_string(&bf).unwrap();
    println!("JSON: {}", json);

    let restored: BloomFilter = serde_json::from_str(&json).unwrap();
    assert!(restored.contains(&"alpha"));
}
```

### Bincode (compact) example
```rust
use bloomz::BloomFilter;
#[cfg(feature = "serde")] {
    let mut bf = BloomFilter::new_for_capacity(5_000, 0.01);
    for i in 0..500u32 { bf.insert(&i); }

    let encoded = bincode::serialize(&bf).unwrap();
    let decoded: BloomFilter = bincode::deserialize(&encoded).unwrap();
    assert!(decoded.contains(&42u32));
}
```

### Notes about serde
- The serialized form stores: `m`, `k`, `items`, `words`.
- The hasher builder is recreated with `Default`; changing hash algorithms across serialize/deserialize alters bit interpretation.
- For reproducible bit layout across processes, use the same hasher implementation and seed strategy.

## Set operations
```rust
let mut a = BloomFilter::new_for_capacity(10_000, 0.01);
let mut b = BloomFilter::new_for_capacity(10_000, 0.01);
for i in 0..5000 { a.insert(&i); }
for i in 2500..7500 { b.insert(&i); }

let mut u = a.clone();
u.union_inplace(&b);
let mut inter = a.clone();
inter.intersect_inplace(&b);
```

## Estimating inserts
`approximate_items()` returns how many times `insert` was called (duplicates count). For a true cardinality estimate you would need an additional structure (e.g. HyperLogLog) or track externally.

## Notes
- Bloom filters have *no false negatives* (assuming consistent hashing) but can produce false positives.
- Recreating with a different hasher after a filter is populated invalidates membership queries.
- Parameter quality depends on good hash dispersion; poor hashers increase false positive rate.

## License
MIT (add a LICENSE file if not present).
