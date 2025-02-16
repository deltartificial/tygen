mod parser;
mod visitor;

pub use self::parser::TypeParser;
pub use self::visitor::TypeVisitor;
use crate::error::{Result, TypeTesterError};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TypeInfo {
    pub name: String,
    pub kind: TypeKind,
    pub derives: Vec<String>,
    #[allow(dead_code)]
    pub attributes: Vec<String>,
    pub fields: Vec<FieldInfo>,
    pub manual_impls: Vec<String>,
}

impl TypeInfo {
    pub fn requires_default(&self) -> bool {
        self.derives.iter().any(|d| {
            d.contains("Serialize") || d.contains("Deserialize") || d.contains("PartialEq")
        })
    }

    pub fn requires_serde(&self) -> bool {
        self.derives
            .iter()
            .any(|d| d.contains("Serialize") || d.contains("Deserialize"))
    }

    pub fn has_derive(&self, derive: &str) -> bool {
        self.derives.iter().any(|d| d.contains(derive))
    }

    pub fn has_implementation(&self, trait_name: &str) -> bool {
        self.has_derive(trait_name) || self.manual_impls.iter().any(|i| i.contains(trait_name))
    }
}

#[derive(Debug, Clone)]
pub struct FieldInfo {
    pub name: String,
    pub ty: String,
    #[allow(dead_code)]
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
            if !type_info.has_implementation("Debug") {
                return Err(TypeTesterError::ValidationError(format!(
                    "Type {} must implement Debug",
                    type_info.name
                )));
            }
            if !type_info.has_implementation("Clone") {
                return Err(TypeTesterError::ValidationError(format!(
                    "Type {} must implement Clone",
                    type_info.name
                )));
            }

            if type_info.requires_serde() {
                if !type_info.has_derive("Serialize") || !type_info.has_derive("Deserialize") {
                    return Err(TypeTesterError::ValidationError(format!(
                        "Type {} must derive both Serialize and Deserialize",
                        type_info.name
                    )));
                }
            }

            if type_info.requires_default() && !type_info.has_implementation("Default") {
                return Err(TypeTesterError::ValidationError(format!(
                    "Type {} must implement Default (either derive it or implement it manually)",
                    type_info.name
                )));
            }
        }
        Ok(())
    }
}
