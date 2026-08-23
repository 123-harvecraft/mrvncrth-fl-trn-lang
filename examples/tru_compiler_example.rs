//! TRU Language Compiler Example
//! 
//! Demonstrates how to use the TRU language compiler to translate .tru files
//! to optimized Rust code with performance enhancements.

use trunk_lang::tru_lang::*;
use trunk_lang::performance_prelude::*;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TRU Language Compiler Example ===\n");
    
    // Example 1: Compile TRU code to Rust
    println!("--- Compiling TRU Code ---");
    compile_tru_example()?;
    
    // Example 2: TRU Runtime Performance
    println!("\n--- TRU Runtime Performance ---");
    demonstrate_tru_runtime()?;
    
    // Example 3: TRU Vector Operations
    println!("\n--- TRU Vector Operations ---");
    demonstrate_tru_vectors()?;
    
    // Example 4: TRU Tensor Operations
    println!("\n--- TRU Tensor Operations ---");
    demonstrate_tru_tensors()?;
    
    // Example 5: TRU String Interning
    println!("\n--- TRU String Interning ---");
    demonstrate_tru_strings()?;
    
    // Example 6: TRU Caching
    println!("\n--- TRU Caching ---");
    demonstrate_tru_caching()?;
    
    // Example 7: TRU Functions with Performance Monitoring
    println!("\n--- TRU Functions with Performance Monitoring ---");
    demonstrate_tru_functions()?;
    
    println!("\n=== TRU Language Compiler Example Completed ===");
    Ok(())
}

fn compile_tru_example() -> Result<(), Box<dyn std::error::Error>> {
    // Sample TRU code
    let tru_code = r#"
// TRU Language Sample
use trunk_lang::tru_lang::*;

tru_fn calculate_distance(vec1: &TruVector, vec2: &TruVector) -> f32 {
    let diff = vec1.subtract(vec2);
    let squared_sum = diff.dot_product(&diff);
    squared_sum.sqrt()
}

tru_fn process_batch(vectors: &[TruVector]) -> TruVector {
    let mut result = TruVector::zeros(vectors[0].len());
    
    for vec in vectors {
        result = result.add(vec);
    }
    
    result.scale(1.0 / vectors.len() as f32);
    result
}

tru_fn create_identity_matrix(size: usize) -> TruTensor {
    let mut matrix = TruTensor::zeros(vec![size, size]);
    
    for i in 0..size {
        matrix.set(&[i, i], 1.0);
    }
    
    matrix
}
"#;
    
    // Create TRU compiler
    let compiler = TruCompiler::new()
        .with_optimizations(true)
        .with_simd(true)
        .with_memory_pools(true);
    
    // Compile TRU code to Rust
    let rust_code = compiler.compile(tru_code)?;
    
    println!("Compiled TRU code to Rust:");
    println!("{}", rust_code);
    
    // Save compiled code
    fs::write("examples/compiled_tru.rs", rust_code)?;
    println!("Saved compiled code to: examples/compiled_tru.rs");
    
    Ok(())
}

fn demonstrate_tru_runtime() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = get_tru_runtime();
    
    // Test memory pools
    println!("Testing memory pools...");
    let pool_512 = runtime.get_memory_pool(512).unwrap();
    let buffer1 = pool_512.get();
    let buffer2 = pool_512.get();
    
    println!("Got buffers from pool: {} bytes each", buffer1.len());
    
    // Return buffers to pool
    pool_512.return_item(buffer1);
    pool_512.return_item(buffer2);
    
    // Test performance metrics
    println!("Testing performance metrics...");
    runtime.record_metric("test_operation", 1000); // 1 microsecond
    runtime.record_metric("test_operation", 2000); // 2 microseconds
    runtime.record_metric("test_operation", 500);  // 0.5 microseconds
    
    if let Some(stats) = runtime.get_metric("test_operation") {
        println!("Performance stats for test_operation:");
        println!("  Call count: {}", stats.call_count);
        println!("  Avg time: {} ns", stats.avg_time_ns);
        println!("  Min time: {} ns", stats.min_time_ns);
        println!("  Max time: {} ns", stats.max_time_ns);
        println!("  Ops/sec: {:.0}", stats.ops_per_sec);
    }
    
    Ok(())
}

fn demonstrate_tru_vectors() -> Result<(), Box<dyn std::error::Error>> {
    // Create vectors
    let mut vec1 = TruVector::new(vec![1.0, 2.0, 3.0, 4.0]);
    let vec2 = TruVector::new(vec![0.5, 1.5, 2.5, 3.5]);
    let vec3 = TruVector::random(1000);
    
    println!("Vector operations:");
    println!("  vec1 length: {}", vec1.len());
    println!("  vec2 length: {}", vec2.len());
    println!("  vec3 length: {}", vec3.len());
    
    // Vector operations
    let dot_product = vec1.dot_product(&vec2);
    let cosine_sim = vec1.cosine_similarity(&vec2);
    let sum = vec1.add(&vec2);
    let diff = vec1.subtract(&vec2);
    
    println!("  Dot product: {}", dot_product);
    println!("  Cosine similarity: {:.4}", cosine_sim);
    println!("  Sum vector length: {}", sum.len());
    println!("  Difference vector length: {}", diff.len());
    
    // Vector normalization
    vec1.normalize();
    println!("  Normalized vec1 norm: {:.6}", vec1.norm());
    
    // Vector scaling
    vec2.scale(2.0);
    println!("  Scaled vec2 first element: {}", vec2.as_slice()[0]);
    
    Ok(())
}

fn demonstrate_tru_tensors() -> Result<(), Box<dyn std::error::Error>> {
    // Create tensors
    let tensor1 = TruTensor::new(vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0], vec![2, 3]);
    let tensor2 = TruTensor::zeros(vec![3, 3]);
    let tensor3 = TruTensor::ones(vec![2, 2, 2]);
    let tensor4 = TruTensor::random(vec![4, 4]);
    
    println!("Tensor operations:");
    println!("  tensor1 shape: {:?}", tensor1.shape());
    println!("  tensor2 shape: {:?}", tensor2.shape());
    println!("  tensor3 shape: {:?}", tensor3.shape());
    println!("  tensor4 shape: {:?}", tensor4.shape());
    
    // Tensor element access
    println!("  tensor1[0,0]: {}", tensor1.get(&[0, 0]).unwrap_or(0.0));
    println!("  tensor1[1,2]: {}", tensor1.get(&[1, 2]).unwrap_or(0.0));
    
    // Tensor element modification
    let mut tensor5 = tensor2.clone();
    tensor5.set(&[0, 0], 42.0);
    tensor5.set(&[1, 1], 24.0);
    println!("  tensor5[0,0]: {}", tensor5.get(&[0, 0]).unwrap_or(0.0));
    println!("  tensor5[1,1]: {}", tensor5.get(&[1, 1]).unwrap_or(0.0));
    
    // Tensor reshaping
    let flat_data = vec![1.0; 12];
    let tensor6 = TruTensor::new(flat_data, vec![2, 2, 3]);
    let reshaped = tensor6.reshape(vec![3, 4]).unwrap();
    println!("  Original shape: {:?}", tensor6.shape());
    println!("  Reshaped to: {:?}", reshaped.shape());
    
    // Tensor flattening
    let flattened = tensor6.flatten();
    println!("  Flattened vector length: {}", flattened.len());
    
    Ok(())
}

fn demonstrate_tru_strings() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = get_tru_runtime();
    
    // Create TRU strings with interning
    let text1 = TruString::new("hello world", runtime.string_interner.clone());
    let text2 = TruString::new("tru language", runtime.string_interner.clone());
    let text3 = TruString::new("hello world", runtime.string_interner.clone()); // Duplicate
    
    println!("String interning:");
    println!("  text1 id: {}", text1.id());
    println!("  text2 id: {}", text2.id());
    println!("  text3 id: {}", text3.id());
    println!("  text1 == text3: {}", text1 == text3); // Should be true (same interned string)
    println!("  text1 == text2: {}", text1 == text2); // Should be false
    
    // Retrieve interned strings
    println!("  text1 content: {:?}", text1.as_str());
    println!("  text2 content: {:?}", text2.as_str());
    println!("  text3 content: {:?}", text3.as_str());
    
    Ok(())
}

fn demonstrate_tru_caching() -> Result<(), Box<dyn std::error::Error>> {
    let runtime = get_tru_runtime();
    
    // Test cache performance
    println!("Testing cache performance...");
    
    // Cache some data
    for i in 0..1000 {
        let key = format!("key_{}", i);
        let value = format!("value_{}", i);
        let serialized = value.as_bytes().to_vec();
        runtime.cache.put(key, serialized);
    }
    
    // Test cache hits
    let start = std::time::Instant::now();
    let mut hits = 0;
    
    for i in 0..1000 {
        let key = format!("key_{}", i);
        if runtime.cache.get(&key).is_some() {
            hits += 1;
        }
    }
    
    let cache_duration = start.elapsed();
    
    println!("  Cache operations: 1000 reads in {:?}", cache_duration);
    println!("  Cache hits: {} / 1000", hits);
    println!("  Hit rate: {:.2}%", hits as f64 / 1000.0 * 100.0);
    println!("  Read throughput: {:.0} ops/sec", 1000.0 / cache_duration.as_secs_f64());
    
    Ok(())
}

fn demonstrate_tru_functions() -> Result<(), Box<dyn std::error::Error>> {
    // Create TRU functions with performance monitoring
    let expensive_function = tru_fn!("expensive_operation", {
        // Simulate expensive computation
        let mut sum = 0.0;
        for i in 0..100000 {
            sum += (i as f32).sin();
        }
        sum
    });
    
    let vector_function = tru_fn!("vector_creation", {
        TruVector::random(1000)
    });
    
    let tensor_function = tru_fn!("tensor_creation", {
        TruTensor::random(vec![10, 10])
    });
    
    // Call functions multiple times
    println!("Calling TRU functions...");
    
    for _ in 0..100 {
        let _ = expensive_function.call();
        let _ = vector_function.call();
        let _ = tensor_function.call();
    }
    
    // Get performance statistics
    if let Some(stats) = expensive_function.get_stats() {
        println!("Expensive operation stats:");
        println!("  Calls: {}", stats.call_count);
        println!("  Avg time: {} ns", stats.avg_time_ns);
        println!("  Ops/sec: {:.0}", stats.ops_per_sec);
    }
    
    if let Some(stats) = vector_function.get_stats() {
        println!("Vector creation stats:");
        println!("  Calls: {}", stats.call_count);
        println!("  Avg time: {} ns", stats.avg_time_ns);
        println!("  Ops/sec: {:.0}", stats.ops_per_sec);
    }
    
    if let Some(stats) = tensor_function.get_stats() {
        println!("Tensor creation stats:");
        println!("  Calls: {}", stats.call_count);
        println!("  Avg time: {} ns", stats.avg_time_ns);
        println!("  Ops/sec: {:.0}", stats.ops_per_sec);
    }
    
    Ok(())
}
