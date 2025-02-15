mod derive;
mod serialization;
mod size;
mod field;

use crate::analyzer::TypeInfo;
use crate::error::Result;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub use self::derive::DeriveTestGenerator;
pub use self::serialization::SerializationTestGenerator;
pub use self::size::SizeTestGenerator;
pub use self::field::FieldTestGenerator;

#[derive(Default)]
pub struct TestConfig {
    pub check_derives: bool,
    pub check_serialization: bool,
    pub check_size: bool,
    pub check_fields: bool,
}

pub trait TestGenerator {
    fn generate(&self, type_info: &TypeInfo) -> TokenStream;
    fn is_applicable(&self, type_info: &TypeInfo) -> bool;
}

pub struct TestSuite {
    generators: Vec<Box<dyn TestGenerator>>,
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
            config: TestConfig {
                check_derives: true,
                check_serialization: true,
                check_size: true,
                check_fields: true,
            },
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

    pub fn generate_tests(&self, types: &[TypeInfo]) -> Result<String> {
        let mut test_stream = TokenStream::new();

        for type_info in types {
            let test_name = format_ident!("test_{}", type_info.name.to_lowercase());
            
            let mut type_tests = TokenStream::new();
            for generator in &self.generators {
                if generator.is_applicable(type_info) {
                    type_tests.extend(generator.generate(type_info));
                }
            }

            let module = quote! {
                #[cfg(test)]
                mod #test_name {
                    use super::*;
                    use serde_json;
                    use std::mem;

                    #type_tests
                }
            };

            test_stream.extend(module);
        }

        Ok(test_stream.to_string())
    }
} 