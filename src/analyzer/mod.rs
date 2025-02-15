mod parser;
mod visitor;

use crate::error::{Result, TypeTesterError};
use std::path::Path;
pub use self::parser::TypeParser;
pub use self::visitor::TypeVisitor;

#[derive(Debug)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub derives: Vec<String>,
    pub attributes: Vec<String>,
}

#[derive(Debug)]
pub enum TypeKind {
    Struct,
    Enum,
}

pub struct TypeAnalyzer {
    parser: TypeParser,
    visitor: TypeVisitor,
}

impl TypeAnalyzer {
    pub fn new() -> Self {
        Self {
            parser: TypeParser::new(),
            visitor: TypeVisitor::new(),
        }
    }

    pub fn analyze_file<P: AsRef<Path>>(&self, path: P) -> Result<Vec<TypeInfo>> {
        let content = std::fs::read_to_string(path)?;
        let syntax = self.parser.parse_file(&content)?;
        self.visitor.visit_file(&syntax)
    }
} 