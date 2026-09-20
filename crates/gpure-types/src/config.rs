use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub entry_point: PathBuf,
    pub output_dir: PathBuf,
    pub mode: BuildMode,
    #[serde(default = "default_minify")]
    pub minify: bool,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            entry_point: PathBuf::from("src/main.rs"),
            output_dir: PathBuf::from("dist"),
            mode: BuildMode::Development,
            minify: false,
        }
    }
}

fn default_minify() -> bool { false }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BuildMode {
    Development,
    Production,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_build_config() {
        let config = BuildConfig::default();
        assert_eq!(config.entry_point.to_str().unwrap(), "src/main.rs");
        assert_eq!(config.output_dir.to_str().unwrap(), "dist");
        assert_eq!(config.mode, BuildMode::Development);
        assert!(!config.minify);
    }

    #[test]
    fn test_build_config_serialization() {
        let config = BuildConfig {
            entry_point: std::path::PathBuf::from("app/main.ts"),
            output_dir: std::path::PathBuf::from("build"),
            mode: BuildMode::Production,
            minify: true,
        };

        // serialization to JSON
        let json = serde_json::to_string(&config).unwrap();
        
        // deserialization again
        let deserialized: BuildConfig = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.mode, BuildMode::Production);
        assert!(deserialized.minify);
    }
}