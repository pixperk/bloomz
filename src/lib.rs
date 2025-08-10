//! bloomz: a fast, flexible bloom filter for rust
#![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "std")]
extern crate std;

/// A bitset implementation for the bloom filter.
pub mod bitset;
/// Mathematical functions for calculating optimal bloom filter parameters.
pub mod math;
/// Hashing functions for the bloom filter.
pub mod hashing;
/// The bloom filter implementation.
pub mod bloom;

pub use bloom::BloomFilter;