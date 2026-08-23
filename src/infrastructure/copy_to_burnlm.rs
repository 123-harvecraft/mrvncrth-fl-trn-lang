//! Copy to BurnLM Infrastructure
//! 
//! Provides utilities for copying models to BurnLM format.

use std::path::Path;
use std::fs;
use anyhow::Result;

/// Copy model files to BurnLM format
pub fn copy_to_burnlm(source_path: &str, target_path: &str) -> Result<()> {
    println!("[INFRA] Copying model to BurnLM format...");
    
    let source_dir = Path::new(source_path);
    let target_dir = Path::new(target_path);
    
    // Create target directory if it doesn't exist
    fs::create_dir_all(target_dir)?;
    
    // Copy essential model files
    let files_to_copy = vec![
        "config.json",
        "tokenizer.json", 
        "vocab.txt",
        "model.safetensors",
        "model.bin",
    ];
    
    for file in files_to_copy {
        let source_file = source_dir.join(file);
        let target_file = target_dir.join(file);
        
        if source_file.exists() {
            fs::copy(&source_file, &target_file)?;
            println!("[OK] Copied {} to BurnLM", file);
        } else {
            println!("[WARN] {} not found, skipping", file);
        }
    }
    
    // Create BurnLM-specific configuration
    let burnlm_config = serde_json::json!({
        "model_type": "burnlm",
        "source_path": source_path,
        "target_path": target_path,
        "format": "safetensors",
        "backend": "wgpu",
        "precision": "f32"
    });
    
    fs::write(
        target_dir.join("burnlm_config.json"),
        serde_json::to_string_pretty(&burnlm_config)?
    )?;
    
    println!("[OK] BurnLM configuration created");
    println!("[OK] Model copied to BurnLM format successfully");
    
    Ok(())
}

/// Validate BurnLM compatibility
pub fn validate_burnlm_compatibility(model_path: &str) -> Result<bool> {
    let model_dir = Path::new(model_path);
    
    let required_files = vec![
        "config.json",
        "model.safetensors"
    ];
    
    for file in required_files {
        if !model_dir.join(file).exists() {
            println!("[ERROR] Required file {} not found", file);
            return Ok(false);
        }
    }
    
    println!("[OK] Model is BurnLM compatible");
    Ok(true)
}
