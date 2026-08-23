//! Performance Comparison Example
//! 
//! Demonstrates the performance improvements between original and optimized versions

use std::time::Instant;
use trunk_lang::trunk_lang_core::*;
use trunk_lang::utils::secureUtils_optimized::*;
use trunk_lang::utils::helperUtils_optimized::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Trunk Lang Performance Comparison ===\n");
    
    // Initialize Trunk Lang core
    let trunk = TrunkLang::new();
    
    // Test 1: UUID Generation Performance
    println!("--- UUID Generation Performance ---");
    test_uuid_performance(&trunk)?;
    
    // Test 2: String Interning Performance
    println!("\n--- String Interning Performance ---");
    test_string_interning(&trunk)?;
    
    // Test 3: Encryption Performance
    println!("\n--- Encryption Performance ---");
    test_encryption_performance()?;
    
    // Test 4: Embedding Generation Performance
    println!("\n--- Embedding Generation Performance ---");
    test_embedding_performance()?;
    
    // Test 5: Vector Operations Performance
    println!("\n--- Vector Operations Performance ---");
    test_vector_operations_performance()?;
    
    // Test 6: Cache Performance
    println!("\n--- Cache Performance ---");
    test_cache_performance()?;
    
    // Show performance metrics
    println!("\n--- Performance Metrics ---");
    show_performance_metrics(&trunk);
    
    println!("\n=== Performance comparison completed! ===");
    Ok(())
}

fn test_uuid_performance(trunk: &TrunkLang) -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 100_000;
    
    // Test optimized UUID generation
    let start = Instant::now();
    let mut optimized_uuids = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        optimized_uuids.push(trunk.generate_uuid());
    }
    let optimized_duration = start.elapsed();
    
    // Test standard UUID generation
    let start = Instant::now();
    let mut standard_uuids = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        standard_uuids.push(trunk.uuid_gen.generate_standard());
    }
    let standard_duration = start.elapsed();
    
    println!("Optimized UUID generation ({}): {:?}", ITERATIONS, optimized_duration);
    println!("Standard UUID generation ({}): {:?}", ITERATIONS, standard_duration);
    println!("Speedup: {:.2}x", standard_duration.as_nanos() as f64 / optimized_duration.as_nanos() as f64);
    
    // Verify uniqueness
    optimized_uuids.sort();
    standard_uuids.sort();
    
    let unique_optimized = optimized_uuids.iter().zip(optimized_uuids.iter().skip(1))
        .filter(|(a, b)| a != b)
        .count();
    let unique_standard = standard_uuids.iter().zip(standard_uuids.iter().skip(1))
        .filter(|(a, b)| a != b)
        .count();
    
    println!("Uniqueness - Optimized: {} / {}, Standard: {} / {}", 
             unique_optimized, ITERATIONS, unique_standard, ITERATIONS);
    
    Ok(())
}

fn test_string_interning(trunk: &TrunkLang) -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 50_000;
    const UNIQUE_STRINGS: usize = 1_000;
    
    // Generate test strings
    let test_strings: Vec<String> = (0..UNIQUE_STRINGS)
        .map(|i| format!("test_string_{}", i))
        .collect();
    
    // Test string interning
    let start = Instant::now();
    let mut interned_ids = Vec::with_capacity(ITERATIONS);
    for i in 0..ITERATIONS {
        let string_index = i % UNIQUE_STRINGS;
        interned_ids.push(trunk.intern_string(&test_strings[string_index]));
    }
    let interning_duration = start.elapsed();
    
    // Test string retrieval
    let start = Instant::now();
    let mut retrieved_strings = Vec::with_capacity(ITERATIONS);
    for &id in &interned_ids {
        if let Some(s) = trunk.get_interned_string(id) {
            retrieved_strings.push(s);
        }
    }
    let retrieval_duration = start.elapsed();
    
    println!("String interning ({} ops): {:?}", ITERATIONS, interning_duration);
    println!("String retrieval ({} ops): {:?}", ITERATIONS, retrieval_duration);
    println!("Average interning time: {:.2} ns", interning_duration.as_nanos() as f64 / ITERATIONS as f64);
    println!("Average retrieval time: {:.2} ns", retrieval_duration.as_nanos() as f64 / ITERATIONS as f64);
    
    Ok(())
}

fn test_encryption_performance() -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 10_000;
    let test_data = "This is test data for encryption performance benchmark";
    let key = b"development_key_not_for_production_32";
    
    // Test optimized encryption
    let start = Instant::now();
    let mut encrypted_data = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        encrypted_data.push(encrypt_data_optimized(key, test_data.as_bytes()));
    }
    let encryption_duration = start.elapsed();
    
    // Test optimized decryption
    let start = Instant::now();
    let mut decrypted_success = 0;
    for encrypted in &encrypted_data {
        if decrypt_data_optimized(key, encrypted).is_ok() {
            decrypted_success += 1;
        }
    }
    let decryption_duration = start.elapsed();
    
    println!("Optimized encryption ({} ops): {:?}", ITERATIONS, encryption_duration);
    println!("Optimized decryption ({} ops): {:?}", ITERATIONS, decryption_duration);
    println!("Successful decryptions: {} / {}", decrypted_success, ITERATIONS);
    println!("Encryption throughput: {:.2} ops/sec", ITERATIONS as f64 / encryption_duration.as_secs_f64());
    println!("Decryption throughput: {:.2} ops/sec", ITERATIONS as f64 / decryption_duration.as_secs_f64());
    
    Ok(())
}

fn test_embedding_performance() -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 5_000;
    const DIM: usize = 512;
    let test_text = "This is a test text for embedding generation performance benchmark";
    
    // Test optimized embedding generation
    let start = Instant::now();
    let mut embeddings = Vec::with_capacity(ITERATIONS);
    for _ in 0..ITERATIONS {
        embeddings.push(utils_embedding_vec_dim_optimized(test_text, DIM));
    }
    let embedding_duration = start.elapsed();
    
    // Test batch embedding generation
    let test_texts: Vec<&str> = (0..ITERATIONS)
        .map(|_| test_text)
        .collect();
    
    let start = Instant::now();
    let batch_embeddings = utils_embedding_vec_batch(&test_texts, DIM);
    let batch_duration = start.elapsed();
    
    println!("Individual embedding generation ({} ops, dim={}): {:?}", ITERATIONS, DIM, embedding_duration);
    println!("Batch embedding generation ({} ops, dim={}): {:?}", ITERATIONS, DIM, batch_duration);
    println!("Batch speedup: {:.2}x", embedding_duration.as_nanos() as f64 / batch_duration.as_nanos() as f64);
    
    // Verify consistency
    if let Some(first_embedding) = embeddings.first() {
        let consistent = embeddings.iter().all(|e| e == first_embedding);
        println!("Embeddings consistent: {}", consistent);
    }
    
    Ok(())
}

fn test_vector_operations_performance() -> Result<(), Box<dyn std::error::Error>> {
    const ITERATIONS: usize = 100_000;
    const DIM: usize = 256;
    
    // Generate test vectors
    let vectors_a: Vec<Vec<f32>> = (0..ITERATIONS)
        .map(|_| (0..DIM).map(|i| i as f32 / DIM as f32).collect())
        .collect();
    let vectors_b: Vec<Vec<f32>> = (0..ITERATIONS)
        .map(|_| (0..DIM).map(|i| (DIM - i) as f32 / DIM as f32).collect())
        .collect();
    
    // Test dot product
    let start = Instant::now();
    let mut dot_products = Vec::with_capacity(ITERATIONS);
    for (a, b) in vectors_a.iter().zip(vectors_b.iter()) {
        dot_products.push(VectorOps::dot_product(a, b));
    }
    let dot_duration = start.elapsed();
    
    // Test batch dot product
    let start = Instant::now();
    let batch_dot_products = VectorOps::batch_dot_product(
        &vectors_a.iter().map(|v| v.as_slice()).collect::<Vec<_>>(),
        &vectors_b.iter().map(|v| v.as_slice()).collect::<Vec<_>>(),
    );
    let batch_dot_duration = start.elapsed();
    
    // Test cosine similarity
    let start = Instant::now();
    let mut similarities = Vec::with_capacity(ITERATIONS);
    for (a, b) in vectors_a.iter().zip(vectors_b.iter()) {
        similarities.push(VectorOps::cosine_similarity(a, b));
    }
    let similarity_duration = start.elapsed();
    
    println!("Dot product ({} ops, dim={}): {:?}", ITERATIONS, DIM, dot_duration);
    println!("Batch dot product ({} ops, dim={}): {:?}", ITERATIONS, DIM, batch_dot_duration);
    println!("Cosine similarity ({} ops, dim={}): {:?}", ITERATIONS, DIM, similarity_duration);
    println!("Batch speedup: {:.2}x", dot_duration.as_nanos() as f64 / batch_dot_duration.as_nanos() as f64);
    
    // Verify results consistency
    let dot_consistent = dot_products == batch_dot_products;
    println!("Dot product results consistent: {}", dot_consistent);
    
    Ok(())
}

fn test_cache_performance() -> Result<(), Box<dyn std::error::Error>> {
    const OPERATIONS: usize = 1_000_000;
    const CACHE_SIZE: usize = 10_000;
    
    let cache = FastCache::<String, String>::new(CACHE_SIZE);
    
    // Test cache writes
    let start = Instant::now();
    for i in 0..OPERATIONS {
        let key = format!("key_{}", i % CACHE_SIZE);
        let value = format!("value_{}", i);
        cache.put(key, value);
    }
    let write_duration = start.elapsed();
    
    // Test cache reads
    let start = Instant::now();
    let mut hits = 0;
    for i in 0..OPERATIONS {
        let key = format!("key_{}", i % CACHE_SIZE);
        if cache.get(&key).is_some() {
            hits += 1;
        }
    }
    let read_duration = start.elapsed();
    
    println!("Cache writes ({} ops): {:?}", OPERATIONS, write_duration);
    println!("Cache reads ({} ops): {:?}", OPERATIONS, read_duration);
    println!("Cache hits: {} / {}", hits, OPERATIONS);
    println!("Hit rate: {:.2}%", hits as f64 / OPERATIONS as f64 * 100.0);
    println!("Write throughput: {:.2} ops/sec", OPERATIONS as f64 / write_duration.as_secs_f64());
    println!("Read throughput: {:.2} ops/sec", OPERATIONS as f64 / read_duration.as_secs_f64());
    
    Ok(())
}

fn show_performance_metrics(trunk: &TrunkLang) {
    let operations = vec![
        "intern_string",
        "get_interned_string", 
        "generate_uuid",
    ];
    
    for operation in operations {
        if let Some(metrics) = trunk.performance_metrics(operation) {
            println!("{}:", operation);
            println!("  Operations/sec: {:.0}", metrics.operations_per_second);
            println!("  Avg latency: {:.2} ms", metrics.average_latency_ms);
        }
    }
}
