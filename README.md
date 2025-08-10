# 🌸 Bloomz

**Fast, flexible Bloom filter for Rust with pluggable hashers and parallel operations.**

[![Crates.io](https://img.shields.io/crates/v/bloomz.svg)](https://crates.io/crates/bloomz)
[![Documentation](https://docs.rs/bloomz/badge.svg)](https://docs.rs/bloomz)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Features

- **Fast**: Optimized bit operations with efficient double hashing
- **Flexible**: Pluggable hash builders (SipHash, AHash, xxHash, etc.)
- **Parallel**: Batch operations with Rayon for multi-core performance  
- **Serializable**: JSON and binary serialization with Serde
- **Safe**: No unsafe code, extensive testing

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
bloomz = "0.1"

# Enable optional features
bloomz = { version = "0.1", features = ["serde", "rayon"] }
```

### Basic Usage

```rust
use bloomz::BloomFilter;

// Create a filter for ~1000 items with 1% false positive rate
let mut filter = BloomFilter::new_for_capacity(1000, 0.01);

// Insert items
filter.insert(&"hello");
filter.insert(&42);

// Check membership
assert!(filter.contains(&"hello"));
assert!(!filter.contains(&"world"));
```

### Parallel Operations (with `rayon` feature)

```rust
use bloomz::BloomFilter;
use rayon::prelude::*;
use std::collections::hash_map::RandomState;

let rs = RandomState::new();
let mut filter = BloomFilter::with_hasher(10000, 7, rs);

// Parallel batch insert
let items: Vec<i32> = (0..1000).collect();
filter.insert_batch(items.par_iter().cloned());

// Parallel batch contains
let test_items: Vec<i32> = (500..600).collect();
let results = filter.contains_batch(test_items.par_iter().cloned());

// Check if all items are present
let all_present = filter.contains_all(test_items.par_iter().cloned());
```

### Serialization (with `serde` feature)

```rust
use bloomz::BloomFilter;

let mut filter = BloomFilter::new_for_capacity(100, 0.01);
filter.insert(&"data");

// JSON serialization
let json = serde_json::to_string(&filter)?;
let restored: BloomFilter = serde_json::from_str(&json)?;

// Binary serialization  
let bytes = filter.to_bytes();
let restored = BloomFilter::from_bytes(&bytes).unwrap();
```

### Custom Hash Builders

```rust
use bloomz::BloomFilter;
use std::collections::hash_map::RandomState;

// Default SipHash (secure)
let filter1 = BloomFilter::new(1000, 5);

// Custom RandomState
let rs = RandomState::new();
let filter2 = BloomFilter::with_hasher(1000, 5, rs);

// Fast hashers (requires feature flags)
#[cfg(feature = "fast-ahash")]
{
    use ahash::AHasher;
    let filter3 = BloomFilter::with_hasher(1000, 5, 
        ahash::RandomState::new());
}
```

## Performance

Bloomz uses several optimizations:

- **Double Hashing**: Generate k hash functions from just 2 base hashes
- **Efficient Bit Operations**: Word-aligned bit manipulation with `u64` 
- **Parallel Processing**: Multi-threaded batch operations with Rayon
- **Zero-Copy Serialization**: Direct bit vector serialization

### Benchmarks

Run benchmarks to compare hashers and parallel vs sequential operations:

```bash
# Compare different hash builders
cargo bench --features "fast-ahash,fast-xxh3" bloom_hashers

# Compare parallel vs sequential operations  
cargo bench --features rayon parallel_bloom
```

## API Reference

### Core Types

- `BloomFilter<S>` - Main bloom filter with hasher type `S`
- `BitSet` - Underlying bit storage with optimized operations

### Key Methods

#### Insertion
- `insert(&item)` - Insert a single item
- `insert_batch(items)` - Parallel batch insert (rayon feature)

#### Membership
- `contains(&item)` - Check if item is probably in set
- `contains_batch(items)` - Parallel batch check (rayon feature)  
- `contains_all(items)` - Check if all items are present (rayon feature)

#### Set Operations  
- `union_inplace(&other)` - Merge with another filter
- `intersect_inplace(&other)` - Keep only common elements
- `clear()` - Remove all items

#### Serialization
- `to_bytes()` / `from_bytes()` - Binary format
- Serde support for JSON/other formats

### Mathematical Functions

```rust
use bloomz::math;

// Calculate optimal parameters
let m = math::optimal_m(n_items, false_positive_rate);
let k = math::optimal_k(m, n_items);

let filter = BloomFilter::new(m, k);
```

## Feature Flags

| Feature | Description | Dependencies |
|---------|-------------|--------------|
| `serde` | JSON/binary serialization | `serde`, `serde_json` |
| `rayon` | Parallel batch operations | `rayon` |
| `fast-ahash` | AHash hasher support | `ahash` |  
| `fast-xxh3` | xxHash hasher support | `xxhash-rust` |

## Examples

See `src/main.rs` for a complete web crawler URL filter demo:

```bash
# Basic demo
cargo run

# With all features
cargo run --features "rayon,serde,fast-ahash"
```

## Use Cases

- **Web Crawlers**: Avoid revisiting URLs
- **Caching**: Quick "not in cache" checks  
- **Databases**: Reduce disk lookups
- **Networking**: Packet deduplication
- **Analytics**: Unique visitor tracking

## Contributing

Contributions welcome! Please check:

- Run `cargo test --all-features` 
- Run `cargo bench --all-features`
- Add tests for new features
- Update documentation

## License

MIT License - see [LICENSE](LICENSE) file.

---

🌸 **Bloomz**: Where speed meets flexibility in Rust Bloom filters!
