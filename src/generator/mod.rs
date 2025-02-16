mod derive;
mod field;
mod serialization;
mod size;

use crate::analyzer::TypeInfo;
use crate::error::{Result, TypeTesterError};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

pub use self::derive::DeriveTestGenerator;
pub use self::field::FieldTestGenerator;
pub use self::serialization::SerializationTestGenerator;
pub use self::size::SizeTestGenerator;

#[derive(Default)]
pub struct TestConfig {
    #[allow(dead_code)]
    pub check_derives: bool,
    #[allow(dead_code)]
    pub check_serialization: bool,
    #[allow(dead_code)]
    pub check_size: bool,
    #[allow(dead_code)]
    pub check_fields: bool,
}

pub trait TestGenerator {
    fn generate(&self, type_info: &TypeInfo) -> TokenStream;
    fn is_applicable(&self, type_info: &TypeInfo) -> bool;
    fn required_imports(&self) -> Vec<&'static str> {
        Vec::new()
    }
    fn generator_type(&self) -> &'static str {
        "default"
    }
}

pub struct TestSuite {
    generators: Vec<Box<dyn TestGenerator>>,
    #[allow(dead_code)]
    config: TestConfig,
}

impl Default for TestSuite {
    fn default() -> Self {
        Self {
            generators: vec![
                Box::new(DeriveTestGenerator::new()),
                Box::new(SerializationTestGenerator::new()),
                Box::new(SizeTestGenerator::new()),
                Box::new(FieldTestGenerator::new()),
            ],
            config: TestConfig::default(),
        }
    }
}

impl TestSuite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: TestConfig) -> Self {
        Self {
            generators: vec![
                Box::new(DeriveTestGenerator::new()),
                Box::new(SerializationTestGenerator::new()),
                Box::new(SizeTestGenerator::new()),
                Box::new(FieldTestGenerator::new()),
            ],
            config,
        }
    }

    pub fn add_generator(&mut self, generator: Box<dyn TestGenerator>) {
        self.generators.push(generator);
    }

    fn collect_required_imports(&self, types: &[TypeInfo]) -> HashSet<&'static str> {
        let mut imports = HashSet::new();

        for type_info in types {
            for generator in &self.generators {
                if generator.is_applicable(type_info) {
                    imports.extend(generator.required_imports());
                }
            }

            if type_info.requires_serde() {
                imports.extend(vec!["serde", "serde_json"]);
            }
            if type_info.requires_default() {
                imports.insert("std::default::Default");
            }
        }

        imports
    }

    pub fn generate_tests(&self, types: &[TypeInfo]) -> Result<String> {
        if types.is_empty() {
            return Err(TypeTesterError::GenerationError(
                "No types found to generate tests for".to_string(),
            ));
        }

        let mut test_stream = TokenStream::new();
        let type_names: Vec<_> = types.iter().map(|t| format_ident!("{}", &t.name)).collect();
        let imports = self.collect_required_imports(types);
        let import_vec: Vec<_> = imports.into_iter().collect();

        test_stream.extend(quote! {
            use type_tester::types::{#(#type_names),*};
            #(use #import_vec;)*
            use std::default::Default;
        });

        for type_info in types {
            let test_name = format_ident!("test_{}", type_info.name.to_lowercase());
            let mut type_tests = TokenStream::new();

            for generator in &self.generators {
                if generator.is_applicable(type_info) {
                    let should_generate = match generator.generator_type() {
                        "derive" => self.config.check_derives,
                        "serialization" => self.config.check_serialization,
                        "size" => self.config.check_size,
                        "field" => self.config.check_fields,
                        _ => true,
                    };
                    if should_generate {
                        type_tests.extend(generator.generate(type_info));
                    }
                }
            }

            if type_tests.is_empty() {
                return Err(TypeTesterError::GenerationError(format!(
                    "No tests could be generated for type {}",
                    type_info.name
                )));
            }

            let module = quote! {
                #[cfg(test)]
                mod #test_name {
                    use super::*;
                    #type_tests
                }
            };

            test_stream.extend(module);
        }

        Ok(test_stream.to_string())
    }
}
