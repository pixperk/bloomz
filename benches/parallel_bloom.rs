use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bloomz::BloomFilter;
use std::collections::hash_map::RandomState;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

fn bench_insert_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("insert_methods");
    
    let rs = RandomState::new();
    let items: Vec<i32> = (0..10000).collect();
    
    // Sequential insert benchmark
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let mut bf = BloomFilter::with_hasher(100000, 7, rs.clone());
            for item in &items {
                bf.insert(black_box(item));
            }
            bf
        });
    });
    
    // Parallel batch insert benchmark (if rayon feature is enabled)
    #[cfg(feature = "rayon")]
    group.bench_function("parallel_batch", |b| {
        b.iter(|| {
            let mut bf = BloomFilter::with_hasher(100000, 7, rs.clone());
            bf.insert_batch(items.par_iter().cloned());
            bf
        });
    });
    
    group.finish();
}

fn bench_contains_methods(c: &mut Criterion) {
    let mut group = c.benchmark_group("contains_methods");
    
    let rs = RandomState::new();
    let mut bf = BloomFilter::with_hasher(100000, 7, rs.clone());
    
    // Prepare filter with some data
    let insert_items: Vec<i32> = (0..5000).collect();
    for item in &insert_items {
        bf.insert(item);
    }
    
    let test_items: Vec<i32> = (2500..7500).collect(); // Half present, half not
    
    // Sequential contains benchmark
    group.bench_function("sequential", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for item in &test_items {
                results.push(bf.contains(black_box(item)));
            }
            results
        });
    });
    
    // Parallel contains_batch benchmark (if rayon feature is enabled)
    #[cfg(feature = "rayon")]
    group.bench_function("parallel_batch", |b| {
        b.iter(|| bf.contains_batch(test_items.par_iter().cloned()));
    });
    
    // Parallel contains_all benchmark (if rayon feature is enabled)
    #[cfg(feature = "rayon")]
    group.bench_function("parallel_all", |b| {
        b.iter(|| bf.contains_all(test_items.par_iter().cloned()));
    });
    
    group.finish();
}

fn bench_batch_sizes(c: &mut Criterion) {
    #[cfg(feature = "rayon")]
    {
        let mut group = c.benchmark_group("batch_size_scaling");
        
        let rs = RandomState::new();
        let sizes = [100usize, 1000, 10000, 50000];
        
        for &size in &sizes {
            let items: Vec<usize> = (0..size).collect();
            
            group.bench_with_input(BenchmarkId::new("parallel_insert", size), &size, |b, _| {
                b.iter(|| {
                    let mut bf = BloomFilter::with_hasher(size * 10, 7, rs.clone());
                    bf.insert_batch(items.par_iter().cloned());
                    bf
                });
            });
            
            group.bench_with_input(BenchmarkId::new("sequential_insert", size), &size, |b, _| {
                b.iter(|| {
                    let mut bf = BloomFilter::with_hasher(size * 10, 7, rs.clone());
                    for item in &items {
                        bf.insert(black_box(item));
                    }
                    bf
                });
            });
        }
        
        group.finish();
    }
    
    #[cfg(not(feature = "rayon"))]
    {
        let _unused = c; // Suppress warning when rayon is disabled
    }
}

criterion_group!(benches, bench_insert_methods, bench_contains_methods, bench_batch_sizes);
criterion_main!(benches);
