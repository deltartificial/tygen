use crate::error::{Result, TypeTesterError};
use syn::File;

pub struct TypeParser;

impl TypeParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_file(&self, content: &str) -> Result<File> {
        syn::parse_file(content).map_err(|e| TypeTesterError::ParseError(e.to_string()))
    }
}
