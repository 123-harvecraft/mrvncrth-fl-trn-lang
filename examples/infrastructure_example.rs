//! Infrastructure Example
//! 
//! Demonstrates infrastructure functionality of the trunk-lang library

use trunk_lang::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Trunk Lang Infrastructure Example ===");
    
    // Artifactory operations
    println!("\n--- Artifactory Operations ---");
    
    // List repositories
    let repos = infrastructure::list_artifactory_repositories("https://artifactory.example.com")?;
    println!("Available repositories: {:?}", repos);
    
    // Upload model (simulated)
    println!("\nUploading model to artifactory...");
    let upload_result = infrastructure::upload_artifactory(
        "/path/to/model",
        "https://artifactory.example.com",
        "model-repo"
    );
    
    match upload_result {
        Ok(_) => println!("✓ Model upload completed successfully"),
        Err(e) => println!("Upload error: {}", e),
    }
    
    // Download model (simulated)
    println!("\nDownloading model from artifactory...");
    let download_result = infrastructure::download_from_artifactory(
        "https://artifactory.example.com",
        "model-repo",
        "model-name",
        "/target/path"
    );
    
    match download_result {
        Ok(_) => println!("✓ Model download completed successfully"),
        Err(e) => println!("Download error: {}", e),
    }
    
    // BurnLM operations
    println!("\n--- BurnLM Operations ---");
    
    // Copy model to BurnLM format (simulated)
    println!("Copying model to BurnLM format...");
    let burnlm_result = infrastructure::copy_to_burnlm(
        "/source/model/path",
        "/target/burnlm/path"
    );
    
    match burnlm_result {
        Ok(_) => println!("✓ Model copied to BurnLM format successfully"),
        Err(e) => println!("BurnLM copy error: {}", e),
    }
    
    // Validate BurnLM compatibility (simulated)
    println!("Validating BurnLM compatibility...");
    let validation_result = infrastructure::validate_burnlm_compatibility("/path/to/model");
    
    match validation_result {
        Ok(is_compatible) => {
            if is_compatible {
                println!("✓ Model is BurnLM compatible");
            } else {
                println!("⚠ Model is not BurnLM compatible");
            }
        },
        Err(e) => println!("Validation error: {}", e),
    }
    
    // Database connection examples
    println!("\n--- Database Connection Examples ---");
    
    // PostgreSQL connection string
    let postgres_config = Databases::POSTGRES_DB;
    println!("PostgreSQL connection string: {}", postgres_config.get_postgres_connection_string());
    
    // SurrealDB connection
    let surreal_config = Databases::SURREAL_DB;
    println!("SurrealDB URL: {}", surreal_config.get_url());
    println!("SurrealDB namespace: {}", surreal_config.ns);
    println!("SurrealDB database: {}", surreal_config.db);
    
    // Redis connection
    let redis_config = Databases::REDIS_DB;
    println!("Redis URL: {}", redis_config.get_url());
    println!("Redis port: {}", redis_config.port);
    
    // URL connect configurations
    println!("\n--- URL Connect Configurations ---");
    let lib_server = UrlConnect::LIB_SERVER;
    println!("Lib server URL: {}", lib_server.url);
    println!("Lib server port: {}", lib_server.port);
    
    println!("\n=== Infrastructure example completed successfully! ===");
    Ok(())
}
