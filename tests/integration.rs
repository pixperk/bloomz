use bloomz::BloomFilter;
use std::collections::hash_map::RandomState;

#[test]
fn basic_insert_contains() {
    let mut bf = BloomFilter::with_hasher(10_000, 4, RandomState::new());
    bf.insert(&"hello");
    println!("Inserted 'hello'");
    assert!(bf.contains(&"hello"));
    assert!(!bf.contains(&"this-is-ridiculous-and-unseen"));
}

#[test]
fn approximate_items_counts() {
    let rs = RandomState::new();
    let mut bf = BloomFilter::with_hasher(100, 4, rs);
    for i in 0..10 {
        bf.insert(&"dup");
        println!(
            "Insert #{} of 'dup' -> approx_items={} ",
            i + 1,
            bf.approximate_items()
        );
    }
    assert_eq!(bf.approximate_items(), 10, "duplicates should increment");
}

#[test]
fn serialization_roundtrip() {
    let rs = RandomState::new();
    let mut bf = BloomFilter::with_hasher(1_000, 4, rs.clone());
    for i in 0..200u32 {
        bf.insert(&i);
    }
    let bytes = bf.to_bytes();
    println!("Serialized bloom filter: {} bytes", bytes.len());
    // Reuse identical hasher (clone keeps same keys) so hash positions match
    let restored = BloomFilter::from_bytes_hasher(&bytes, rs.clone()).expect("deserialize");
    for i in 0..200u32 {
        assert!(restored.contains(&i), "missing {}", i);
    }
    println!(
        "Roundtrip success; first 5 bytes: {:?}",
        &bytes[..5.min(bytes.len())]
    );
}

#[test]
fn union_and_intersection() {
    let rs = RandomState::new();
    let mut a = BloomFilter::with_hasher(2_000, 4, rs.clone());
    let mut b = BloomFilter::with_hasher(2_000, 4, rs.clone());
    for i in 0..500u32 {
        a.insert(&i);
    }
    for i in 400..900u32 {
        b.insert(&i);
    }

    let mut inter_a = a.clone();
    inter_a.intersect_inplace(&b);
    println!("Intersection computed");
    assert!(inter_a.contains(&450)); // in overlap
    // Note: intersection might have false positives for items not in overlap
    // so we only test that overlap items are definitely present

    let mut union_a = a.clone();
    union_a.union_inplace(&b);
    println!("Union computed");
    assert!(union_a.contains(&100));
    assert!(union_a.contains(&850));
}

#[test]
fn false_positive_rate_reasonable() {
    let rs = RandomState::new();
    let n = 5_000usize;
    let p = 0.01f64;
    let mut bf = BloomFilter::with_hasher(bloomz::math::optimal_m(n, p), bloomz::math::optimal_k(bloomz::math::optimal_m(n, p), n), rs);
    for i in 0..n as u64 {
        bf.insert(&i);
    }

    let trials = 5_000u64;
    let mut fp = 0u64;
    for i in n as u64 .. n as u64 + trials {
        if bf.contains(&i) {
            fp += 1;
        }
    }
    let rate = fp as f64 / trials as f64;
    println!("Observed FP: {} / {} = {:.4}", fp, trials, rate);
    assert!(rate <= p * 5.0 + 0.005, "false positive rate too high: {}", rate);
}

#[test]
#[cfg(feature = "rayon")]
fn test_parallel_operations() {
    use rayon::prelude::*;
    
    let rs = RandomState::new();
    let mut bf = BloomFilter::with_hasher(10000, 7, rs.clone());
    
    // Prepare test data
    let items: Vec<i32> = (0..1000).collect();
    let test_items: Vec<i32> = (500..1500).collect(); // Half overlap with inserted items
    
    // Parallel batch insert
    println!("🔧 Testing parallel batch insert...");
    bf.insert_batch(items.par_iter().cloned());
    
    // Verify all inserted items are found
    for item in &items {
        assert!(bf.contains(item), "Should contain inserted item {}", item);
    }
    
    // Test parallel contains_all (should be false since test_items includes items not inserted)
    println!("🔧 Testing parallel contains_all...");
    let all_present = bf.contains_all(test_items.par_iter().cloned());
    assert!(!all_present, "contains_all should be false for mixed dataset");
    
    // Test parallel contains_batch
    println!("🔧 Testing parallel contains_batch...");
    let results = bf.contains_batch(test_items.par_iter().cloned());
    assert_eq!(results.len(), test_items.len(), "Results length should match input");
    
    // Verify results: items 500-999 should be present, 1000-1499 might have false positives
    let mut found_expected = 0;
    for (i, &present) in results.iter().enumerate() {
        let item = test_items[i];
        if item < 1000 && present {
            found_expected += 1;
        }
    }
    assert!(found_expected > 400, "Should find most of the expected items (found {})", found_expected);
    
    println!("✅ Parallel operations test passed! Found {}/500 expected items", found_expected);
}
