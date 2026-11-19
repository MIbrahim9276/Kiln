use std::path::Path;
use crate::domain::{errors::AppError, manifest::schema::RawManifest};


pub fn load_manifest<P: AsRef<Path>>(file_path: P) -> Result<RawManifest, AppError> {
    let config_content = std::fs::read_to_string(file_path)?;
    let config: RawManifest = toml::from_str(&config_content)?;
    
    Ok(config)
}