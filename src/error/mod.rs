use thiserror::Error;

#[derive(Error, Debug)]
pub enum TypeTesterError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to parse Rust file: {0}")]
    ParseError(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Type validation error: {0}")]
    ValidationError(String),

    #[error("Test generation failed: {0}")]
    GenerationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Syntax error: {0}")]
    SyntaxError(#[from] syn::Error),

    #[error("Type analysis failed: {0}")]
    AnalyzerError(String),

    #[error("Test formatting failed: {0}")]
    FormattingError(String),
}

impl From<String> for TypeTesterError {
    fn from(error: String) -> Self {
        TypeTesterError::GenerationError(error)
    }
}

pub type Result<T> = std::result::Result<T, TypeTesterError>;
