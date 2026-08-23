//! Security Example
//! 
//! Demonstrates security functionality of the trunk-lang library

use trunk_lang::prelude::*;

fn main() {
    println!("=== Trunk Lang Security Example ===");
    
    // API Key operations
    println!("\n--- API Key Operations ---");
    let api_key = secureUtils::generate_api_key();
    println!("Generated API Key: {}", api_key);
    
    let hashed_key = secureUtils::hash_api_key(&api_key);
    println!("Hashed API Key: {}", hashed_key);
    
    let is_valid = secureUtils::validate_api_key(&api_key, &hashed_key);
    println!("API Key validation: {}", is_valid);
    
    // Test with wrong key
    let wrong_key = "wrong-api-key";
    let is_invalid = secureUtils::validate_api_key(&wrong_key, &hashed_key);
    println!("Wrong key validation: {}", is_invalid);
    
    // Encryption/Decryption (if encryption key is set)
    println!("\n--- Encryption/Decryption ---");
    
    // Note: In production, set ENCRYPTION_KEY environment variable
    // For this example, we'll use the default development key
    let test_data = "This is a secret message";
    println!("Original data: {}", test_data);
    
    // Get encryption key (will use default if not set)
    let key = vec![0u8; 32]; // Default key for demonstration
    for (i, byte) in b"development_key_not_for_production_32".iter().enumerate() {
        if i < 32 {
            key[i] = *byte;
        }
    }
    
    // Encrypt data
    let encrypted_data = secureUtils::encrypt_data(&key, test_data.as_bytes());
    println!("Encrypted data: {}", encrypted_data);
    
    // Decrypt data
    let decrypted_data = secureUtils::decrypt_data(&key, &encrypted_data);
    let decrypted_string = String::from_utf8(decrypted_data).expect("Invalid UTF-8");
    println!("Decrypted data: {}", decrypted_string);
    
    // Verify encryption/decryption
    assert_eq!(test_data, decrypted_string);
    println!("✓ Encryption/Decryption successful!");
    
    // Redis result conversion example
    println!("\n--- Redis Result Conversion ---");
    let ok_result = redis::RedisResult::Ok("Success".to_string());
    let converted_ok = secureUtils::convert_redis_result(ok_result);
    println!("OK result conversion: {}", converted_ok);
    
    let err_result = redis::RedisResult::Err(redis::RedisError::from((
        redis::ErrorKind::TypeError,
        "Test error",
    )));
    let converted_err = secureUtils::convert_redis_result(err_result);
    println!("Error result conversion: {}", converted_err);
    
    println!("\n=== Security example completed successfully! ===");
}
