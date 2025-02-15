mod parser;
mod visitor;

use crate::error::Result;
use std::path::Path;
pub use self::parser::TypeParser;
pub use self::visitor::TypeVisitor;

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub derives: Vec<String>,
    pub attributes: Vec<String>,
    pub fields: Vec<FieldInfo>,
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: String,
    pub attributes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
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
        let types = self.visitor.visit_file(&syntax)?;
        self.validate_types(&types)?;
        Ok(types)
    }

    fn validate_types(&self, types: &[TypeInfo]) -> Result<()> {
        for type_info in types {
            if !type_info.derives.contains(&"Debug".to_string()) {
                return Err(crate::error::TypeTesterError::ValidationError(
                    format!("Type {} must derive Debug", type_info.name)
                ));
            }
            if !type_info.derives.contains(&"Clone".to_string()) {
                return Err(crate::error::TypeTesterError::ValidationError(
                    format!("Type {} must derive Clone", type_info.name)
                ));
            }
        }
        Ok(())
    }
} 