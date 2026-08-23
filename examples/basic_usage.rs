//! Basic Usage Example
//! 
//! Demonstrates basic functionality of the trunk-lang library

use trunk_lang::prelude::*;

fn main() {
    println!("=== Trunk Lang Basic Usage Example ===");
    
    // Utility functions
    println!("\n--- Utility Functions ---");
    let current_time = utils::current_time();
    println!("Current time: {}", current_time);
    
    let current_date = utils::current_time_ymd();
    println!("Current date (YMD): {}", current_date);
    
    let uuid = utils::uniqueIdUUID();
    println!("Generated UUID: {}", uuid);
    
    let counter = utils::generateidcounter();
    println!("Generated counter: {}", counter);
    
    // Database configurations
    println!("\n--- Database Configurations ---");
    let postgres_db = Databases::POSTGRES_DB;
    println!("PostgreSQL DB name: {}", postgres_db.name);
    println!("PostgreSQL connection: {}", postgres_db.get_postgres_connection_string());
    
    let surreal_db = Databases::SURREAL_DB;
    println!("SurrealDB URL: {}", surreal_db.get_url());
    
    // LLM configurations
    println!("\n--- LLM Configurations ---");
    let asist_model = LLM::ASIST;
    println!("ASIST Model: {} ({})", asist_model.code, asist_model.description);
    
    let ais_model = LLM::AIS;
    println!("AIS Model: {} ({})", ais_model.code, ais_model.description);
    
    // Size dimensions
    println!("\n--- Size Dimensions ---");
    let dim_2048 = SizeDim::SIZE_DIM_2048;
    println!("2048 dimension: {}", dim_2048.size);
    
    // Status processes
    println!("\n--- Status Processes ---");
    let ready_status = StatusProcess::READY;
    println!("Ready status: {} ({})", ready_status.status, ready_status.description);
    
    // LLM status
    println!("\n--- LLM Status ---");
    let llm_ready = LLMStatus::LLM_READY;
    println!("LLM Ready: {} ({})", llm_ready.status, llm_ready.description);
    
    // Flow execution
    println!("\n--- Flow Execution ---");
    let flow_start = FlowExecute::FLOWSTART;
    println!("Flow start: {} ({})", flow_start.status, flow_start.description);
    
    // User execution types
    println!("\n--- User Execution Types ---");
    let user_type = UserExecute::USER;
    println!("User type: {} ({})", user_type.name, user_type.description);
    
    let system_type = UserExecute::SYSTEM;
    println!("System type: {} ({})", system_type.name, system_type.description);
    
    // Model types
    println!("\n--- Model Types ---");
    let llm_type = LMTYPE::LLM;
    println!("LLM type: {} ({})", llm_type.name, llm_type.description);
    
    let gguf_ext = LMTYPE::namefile_gguf;
    println!("GGUF extension: {} ({})", gguf_ext.name, gguf_ext.description);
    
    // Embedding functions
    println!("\n--- Embedding Functions ---");
    let text = "Hello, Trunk Lang!";
    let embedding = utils::utils_embedding_vec(text);
    println!("Text embedding (3-dim): {:?}", embedding);
    
    let embedding_2048 = utils::utils_embedding_vec_dim(text, 2048);
    println!("Text embedding (2048-dim) length: {}", embedding_2048.len());
    
    // Library info
    println!("\n--- Library Information ---");
    println!("Library name: {}", LIB_NAME);
    println!("Library version: {}", VERSION);
    println!("Library description: {}", LIB_DESCRIPTION);
    
    println!("\n=== Example completed successfully! ===");
}
