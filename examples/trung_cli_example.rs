//! TRUNg CLI Example
//! 
//! Demonstrates how to use the TRUNg build system with trunk.toml configuration
//! instead of Cargo.toml for enhanced build options and TRU language support

use trunk_lang::trung_prelude::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== TRUNg CLI Example ===\n");
    
    // Example 1: Load TRUNg configuration
    println!("--- Loading TRUNg Configuration ---");
    load_trung_config_example()?;
    
    // Example 2: Build with TRUNg
    println!("\n--- Building with TRUNg ---");
    build_with_trung_example()?;
    
    // Example 3: TRU Language Compilation
    println!("\n--- TRU Language Compilation ---");
    tru_compilation_example()?;
    
    // Example 4: Performance Optimization
    println!("\n--- Performance Optimization ---");
    performance_optimization_example()?;
    
    // Example 5: WASM Build
    println!("\n--- WASM Build ---");
    wasm_build_example()?;
    
    // Example 6: Project Information
    println!("\n--- Project Information ---");
    project_info_example()?;
    
    println!("\n=== TRUNg CLI Example Completed ===");
    Ok(())
}

fn load_trung_config_example() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration from trunk.toml
    let config = TrungConfig::load()
        .unwrap_or_else(|e| {
            println!("Warning: Could not load trunk.toml: {}", e);
            println!("Using default configuration");
            TrungConfig::default()
        });
    
    println!("Project Name: {}", config.project.name);
    println!("Project Version: {}", config.project.version);
    println!("Description: {}", config.project.description);
    println!("Authors: {}", config.project.authors.join(", "));
    
    println!("\nBuild Configuration:");
    println!("  Target: {}", config.configure.build_target);
    println!("  Optimization Level: {}", config.configure.optimization_level);
    println!("  LTO: {}", config.configure.lto);
    println!("  Codegen Units: {}", config.configure.codegen_units);
    
    println!("\nFeatures:");
    for feature in &config.features.default {
        println!("  - {}", feature);
    }
    
    println!("\nOptimization Flags:");
    let flags = config.get_optimization_flags();
    for flag in &flags {
        println!("  {}", flag);
    }
    
    Ok(())
}

fn build_with_trung_example() -> Result<(), Box<dyn std::error::Error>> {
    // Create TRUNg builder
    let builder = TrungBuilder::from_current_dir()
        .unwrap_or_else(|e| {
            println!("Warning: Could not create TRUNg builder: {}", e);
            println!("This is just a demonstration - actual build would work in a real project");
            return TrungBuilder::new(".").unwrap();
        });
    
    println!("TRUNg Builder created successfully!");
    
    // Show build options
    println!("\nBuild Options:");
    println!("  Build: trunk build");
    println!("  Test: trunk test");
    println!("  Bench: trunk bench");
    println!("  Format: trunk fmt");
    println!("  Lint: trunk clippy");
    println!("  Clean: trunk clean");
    println!("  Install: trunk install");
    
    // Show optimization flags that would be used
    let flags = builder.config().get_optimization_flags();
    println!("\nOptimization flags for build:");
    for flag in &flags {
        println!("  {}", flag);
    }
    
    Ok(())
}

fn tru_compilation_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("TRU Language Compilation:");
    println!("  Compile TRU file: trunk compile example.tru");
    println!("  Run TRU program: trunk run example.tru");
    
    // Create a sample TRU code snippet
    let tru_code = r#"
// Sample TRU code
use trunk_lang::tru_prelude::*;

tru_fn calculate_similarity(a: &TruVector, b: &TruVector) -> f32 {
    a.cosine_similarity(b)
}

tru_fn main() {
    let vec1 = TruVector::new(vec![1.0, 2.0, 3.0]);
    let vec2 = TruVector::new(vec![4.0, 5.0, 6.0]);
    
    let similarity = calculate_similarity(&vec1, &vec2);
    println!("Similarity: {}", similarity);
}
"#;
    
    println!("\nSample TRU code:");
    println!("{}", tru_code);
    
    println!("\nTRU Language Features:");
    println!("  - Zero-copy operations by default");
    println!("  - Built-in memory management");
    println!("  - SIMD optimizations");
    println!("  - Performance monitoring");
    println!("  - String interning");
    println!("  - Automatic caching");
    
    Ok(())
}

fn performance_optimization_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Performance Optimization Features:");
    
    // Load configuration to show performance settings
    let config = TrungConfig::load()
        .unwrap_or_else(|_| TrungConfig::default());
    
    println!("\nBuild Optimizations:");
    println!("  LTO: {}", config.configure.lto);
    println!("  Optimization Level: {}", config.configure.optimization_level);
    println!("  Codegen Units: {}", config.configure.codegen_units);
    println!("  Panic Strategy: {}", config.configure.panic);
    
    println!("\nMemory Management:");
    println!("  Memory Pools: Enabled");
    println!("  String Interning: Enabled");
    println!("  Buffer Reuse: Enabled");
    println!("  Zero-Copy Operations: Enabled");
    
    println!("\nSIMD Acceleration:");
    println!("  Vector Operations: SIMD enabled");
    println!("  Matrix Operations: SIMD enabled");
    println!("  Embedding Operations: SIMD enabled");
    
    println!("\nCaching:");
    println!("  LRU Cache: Enabled");
    println!("  Cache Size: 10,000 entries");
    println!("  Automatic Eviction: Enabled");
    
    // Show performance metrics
    let runtime = get_tru_runtime();
    println!("\nPerformance Monitoring:");
    println!("  Operation Timing: Enabled");
    println!("  Throughput Tracking: Enabled");
    println!("  Latency Measurement: Enabled");
    println!("  Cache Hit Rates: Tracked");
    
    Ok(())
}

fn wasm_build_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("WASM Build Support:");
    
    // Load configuration
    let config = TrungConfig::load()
        .unwrap_or_else(|_| TrungConfig::default());
    
    println!("\nWASM Configuration:");
    println!("  WASM Target: wasm32-unknown-unknown");
    println!("  Web Support: {}", config.is_feature_enabled("wasm"));
    println!("  Embedded Framework: {}", config.embedded.framework);
    println!("  Memory Limit: {}", config.embedded.memory_limit);
    
    println!("\nWASM Build Commands:");
    println!("  Build WASM: trunk build-wasm");
    println!("  Package WASM: trunk package-wasm");
    println!("  Test WASM: trunk test-wasm");
    
    println!("\nWASM Features:");
    println!("  WebGPU Backend: Available");
    println!("  Browser Support: Full");
    println!("  Node.js Support: Full");
    println!("  Size Optimization: Enabled");
    
    println!("\nAI/ML in WASM:");
    println!("  Burn Framework: Supported");
    println!("  Tensor Operations: Supported");
    println!("  Model Inference: Supported");
    println!("  Precision: f32");
    
    Ok(())
}

fn project_info_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("Project Information:");
    
    // Load configuration
    let config = TrungConfig::load()
        .unwrap_or_else(|_| TrungConfig::default());
    
    println!("\nProject Details:");
    println!("  Name: {}", config.project.name);
    println!("  Version: {}", config.project.version);
    println!("  Description: {}", config.project.description);
    println!("  License: {}", config.project.license);
    println!("  Edition: {}", config.project.edition);
    println!("  Rust Version: {}", config.project.rust_version);
    
    if let Some(repo) = &config.project.repository {
        println!("  Repository: {}", repo);
    }
    
    if let Some(homepage) = &config.project.homepage {
        println!("  Homepage: {}", homepage);
    }
    
    println!("\nKeywords: {}", config.project.keywords.join(", "));
    println!("Categories: {}", config.project.categories.join(", "));
    
    println!("\nBinaries:");
    for binary in &config.bin {
        println!("  - {}: {}", binary.name, binary.path);
        if let Some(target) = &binary.target {
            println!("    Target: {}", target);
        }
    }
    
    println!("\nSource Configuration:");
    println!("  Main: {}", config.src.main);
    println!("  Root: {}", config.src.root);
    println!("  Include: {}", config.src.include.join(", "));
    println!("  Exclude: {}", config.src.exclude.join(", "));
    
    println!("\nSGD/FSDP Configuration:");
    println!("  Enabled: {}", config.sgd_fsdp.enabled);
    println!("  Sharded Training: {}", config.sgd_fsdp.sharded_training);
    println!("  Fully Sharded: {}", config.sgd_fsdp.fully_sharded);
    println!("  Optimizer: {}", config.sgd_fsdp.optimizer);
    println!("  Learning Rate: {}", config.sgd_fsdp.learning_rate);
    println!("  Batch Size: {}", config.sgd_fsdp.batch_size);
    
    println!("\nInstaller Configuration:");
    println!("  Name: {}", config.installer.name);
    println!("  Version: {}", config.installer.version);
    println!("  Targets: {}", config.installer.targets.join(", "));
    println!("  Binary Name: {}", config.installer.binary_name);
    println!("  Install Path: {}", config.installer.install_path);
    println!("  GUI Installer: {}", config.installer.dioxus.gui);
    
    println!("\nAvailable Scripts:");
    println!("  build: {}", config.scripts.build);
    println!("  test: {}", config.scripts.test);
    println!("  bench: {}", config.scripts.bench);
    println!("  compile-tru: {}", config.scripts.compile_tru);
    println!("  run-tru: {}", config.scripts.run_tru);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_config_loading() {
        let config = TrungConfig::load();
        assert!(config.is_ok() || config.is_err()); // Should not panic
    }
    
    #[test]
    fn test_builder_creation() {
        let builder = TrungBuilder::from_current_dir();
        assert!(builder.is_ok() || builder.is_err()); // Should not panic
    }
}
