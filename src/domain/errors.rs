use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Manifest Parsing Error: {0}")]
    ManifestParseError(#[from] toml::de::Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error)
}


impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::ManifestParseError(_) => 3,
            _ => 1
        }
    }
}