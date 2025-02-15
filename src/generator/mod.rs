mod derive;
mod serialization;
mod size;

use crate::analyzer::TypeInfo;
use crate::error::Result;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

pub use self::derive::DeriveTestGenerator;
pub use self::serialization::SerializationTestGenerator;
pub use self::size::SizeTestGenerator;

pub trait TestGenerator {
    fn generate(&self, type_info: &TypeInfo) -> TokenStream;
}

pub struct TestSuite {
    generators: Vec<Box<dyn TestGenerator>>,
}

impl Default for TestSuite {
    fn default() -> Self {
        Self {
            generators: vec![
                Box::new(DeriveTestGenerator::new()),
                Box::new(SerializationTestGenerator::new()),
                Box::new(SizeTestGenerator::new()),
            ],
        }
    }
}

impl TestSuite {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_generator(&mut self, generator: Box<dyn TestGenerator>) {
        self.generators.push(generator);
    }

    pub fn generate_tests(&self, types: &[TypeInfo]) -> Result<String> {
        let mut test_stream = TokenStream::new();

        for type_info in types {
            let type_name = format_ident!("{}", type_info.name);
            let test_name = format_ident!("test_{}", type_info.name.to_lowercase());
            
            let mut type_tests = TokenStream::new();
            for generator in &self.generators {
                type_tests.extend(generator.generate(type_info));
            }

            let module = quote! {
                #[cfg(test)]
                mod #test_name {
                    use super::*;
                    use serde_json;

                    #type_tests
                }
            };

            test_stream.extend(module);
        }

        Ok(test_stream.to_string())
    }
} 