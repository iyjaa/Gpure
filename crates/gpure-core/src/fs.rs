use std::fs;
use std::path::Path;

pub fn read_json<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Failed to read file {}: {}", path, e))?;
    
    serde_json::from_str(&content)
        .map_err(|e| format!("Parse failed JSON {}: {}", path, e))
}

pub fn write_toml<T: serde::Serialize>(path: &str, data: &T) -> Result<(), String> {
    ensure_dir(path)?;
    
    let toml_str = toml::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize TOML: {}", e))?;
    
    fs::write(path, toml_str)
        .map_err(|e| format!("Failed to write file {}: {}", path, e))
}

pub fn ensure_dir(path: &str) -> Result<(), String> {
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory {:?}: {}", parent, e))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct TestConfig {
        name: String,
        version: u32,
    }

    #[test]
    fn test_write_and_read_json() {
        let config = TestConfig {
            name: "gpure".to_string(),
            version: 1,
        };
        
        let path = "/tmp/gpure_test_config.json";
        let json_str = serde_json::to_string(&config).unwrap();
        fs::write(path, &json_str).unwrap();
        
        let result: TestConfig = read_json(path).unwrap();
        assert_eq!(result, config);
        
        fs::remove_file(path).ok();
    }

    #[test]
    fn test_write_toml_creates_dir() {
        let config = TestConfig {
            name: "test".to_string(),
            version: 2,
        };
        
        let path = "/tmp/gpure_test_subdir/config.toml";
        write_toml(path, &config).unwrap();
        
        assert!(Path::new(path).exists());
        
        fs::remove_dir_all("/tmp/gpure_test_subdir").ok();
    }

    #[test]
    fn test_read_json_invalid_path() {
        let result: Result<TestConfig, String> = read_json("/nonexistent/file.json");
        assert!(result.is_err());
    }
}
