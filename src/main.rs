use bloomz::BloomFilter;
use std::collections::hash_map::RandomState;

#[cfg(feature = "rayon")]
use rayon::prelude::*;

fn main() {
    println!("🌸 Bloomz: Fast Bloom Filter Demo");
    
    // Create a Bloom filter for URL deduplication
    let rs = RandomState::new();
    let mut url_filter = BloomFilter::with_hasher(10000, 7, rs.clone());
    
    println!("\n📈 Sequential Operations:");
    
    // Some example URLs that might be crawled
    let urls = vec![
        "https://example.com",
        "https://github.com/rust-lang/rust",
        "https://crates.io",
        "https://doc.rust-lang.org",
        "https://play.rust-lang.org",
    ];
    
    // Insert URLs sequentially
    for url in &urls {
        url_filter.insert(url);
        println!("  ✅ Added: {}", url);
    }
    
    // Check if we've seen these URLs before
    let test_urls = vec![
        "https://example.com",      // Should be found
        "https://new-site.com",     // Should not be found
        "https://crates.io",        // Should be found
    ];
    
    println!("\n🔍 Checking URLs:");
    for url in &test_urls {
        let seen = url_filter.contains(url);
        println!("  {} {}", if seen { "✅" } else { "❌" }, url);
    }
    
    // Demonstrate parallel operations (if rayon feature is enabled)
    #[cfg(feature = "rayon")]
    {
        println!("\n⚡ Parallel Operations:");
        
        let mut parallel_filter = BloomFilter::with_hasher(10000, 7, rs);
        
        // Generate a larger dataset for parallel demo
        let large_dataset: Vec<String> = (0..1000)
            .map(|i| format!("https://site{}.com", i))
            .collect();
        
        println!("  🔧 Batch inserting {} URLs in parallel...", large_dataset.len());
        parallel_filter.insert_batch(large_dataset.par_iter());
        
        // Test parallel contains
        let test_dataset: Vec<String> = (500..600)
            .map(|i| format!("https://site{}.com", i))
            .collect();
        
        println!("  🔍 Checking {} URLs in parallel...", test_dataset.len());
        let results = parallel_filter.contains_batch(test_dataset.par_iter());
        let found_count = results.iter().filter(|&&x| x).count();
        
        println!("  ✅ Found: {}/{} URLs", found_count, test_dataset.len());
        
        // Test contains_all
        let all_present = parallel_filter.contains_all(test_dataset.par_iter());
        println!("  📊 All URLs present: {}", all_present);
    }
    
    #[cfg(not(feature = "rayon"))]
    {
        println!("\n💡 To see parallel operations, run with: cargo run --features rayon");
    }
    
    println!("\n📊 Filter stats:");
    println!("  Items inserted: ~{}", url_filter.approximate_items());
    
    #[cfg(feature = "serde")]
    {
        println!("\n💾 Serialization demo:");
        let json = serde_json::to_string(&url_filter).unwrap();
        println!("  JSON size: {} bytes", json.len());
        
        let bytes = url_filter.to_bytes();
        println!("  Binary size: {} bytes", bytes.len());
    }
    
    println!("\n🎉 Demo complete!");
}
