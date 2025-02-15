use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeTesterError {
    #[error("Failed to read file: {0}")]
    FileReadError(#[from] std::io::Error),

    #[error("Failed to parse Rust file: {0}")]
    ParseError(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Failed to generate tests: {0}")]
    TestGenerationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Syntax error: {0}")]
    SyntaxError(#[from] syn::Error),
}

pub type Result<T> = std::result::Result<T, TypeTesterError>; 