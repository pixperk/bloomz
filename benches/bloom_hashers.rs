use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;

use bloomz::BloomFilter;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

#[cfg(feature = "fast-ahash")]
use ahash::RandomState as AHashState;
#[cfg(feature = "fast-xxh3")]
use xxhash_rust::xxh3::Xxh3Builder as Xxh3BuildHasher;

fn build_and_insert<H: BuildHasher + Clone + 'static>(label: &str, c: &mut Criterion, hasher: H) {
    let n = 50_000u64;
    let m = 400_000; // target bits
    let k = 7;       // typical for this m/n ratio
    c.bench_function(&format!("insert/{label}"), |b| {
        b.iter_batched(
            || BloomFilter::with_hasher(m, k, hasher.clone()),
            |mut bf| {
                for i in 0..n { bf.insert(&i); }
                black_box(bf);
            },
            BatchSize::LargeInput,
        );
    });
}

fn contains_present<H: BuildHasher + Clone + 'static>(label: &str, c: &mut Criterion, hasher: H) {
    let n = 50_000u64;
    let m = 400_000;
    let k = 7;
    let mut bf = BloomFilter::with_hasher(m, k, hasher.clone());
    for i in 0..n { bf.insert(&i); }
    c.bench_function(&format!("contains_present/{label}"), |b| {
        b.iter(|| {
            let mut hits = 0u64;
            for i in 0..n { if bf.contains(&i) { hits += 1; } }
            black_box(hits);
        });
    });
}

fn contains_absent<H: BuildHasher + Clone + 'static>(label: &str, c: &mut Criterion, hasher: H) {
    let n = 50_000u64;
    let m = 400_000;
    let k = 7;
    let mut bf = BloomFilter::with_hasher(m, k, hasher.clone());
    for i in 0..n { bf.insert(&i); }
    c.bench_function(&format!("contains_absent/{label}"), |b| {
        b.iter(|| {
            let mut hits = 0u64;
            for i in n..n * 2 { if bf.contains(&i) { hits += 1; } }
            black_box(hits);
        });
    });
}

fn criterion_benchmark(c: &mut Criterion) {
    // SipHash (RandomState)
    build_and_insert("sip", c, RandomState::new());
    contains_present("sip", c, RandomState::new());
    contains_absent("sip", c, RandomState::new());

    // AHash (feature fast-ahash)
    #[cfg(feature = "fast-ahash")]
    {
        build_and_insert("ahash", c, AHashState::new());
        contains_present("ahash", c, AHashState::new());
        contains_absent("ahash", c, AHashState::new());
    }

    // xxh3 (feature fast-xxh3)
    #[cfg(feature = "fast-xxh3")]
    {
        build_and_insert("xxh3", c, Xxh3BuildHasher::default());
        contains_present("xxh3", c, Xxh3BuildHasher::default());
        contains_absent("xxh3", c, Xxh3BuildHasher::default());
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
