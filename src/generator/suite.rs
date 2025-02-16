use crate::analyzer::TypeInfo;
use crate::error::{Result, TypeTesterError};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashSet;

use super::{
    DeriveTestGenerator, FieldTestGenerator, SerializationTestGenerator, SizeTestGenerator,
    TestConfig, TestGenerator,
};

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
            config: TestConfig::default(),
        }
    }
}

impl TestSuite {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn add_generator(&mut self, generator: Box<dyn TestGenerator>) {
        self.generators.push(generator);
    }

    fn collect_required_imports(&self, types: &[TypeInfo]) -> HashSet<&'static str> {
        let mut imports = HashSet::new();

        for type_info in types {
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
                        imports.extend(generator.required_imports());
                    }
                }
            }

            if type_info.requires_serde() && self.config.check_serialization {
                imports.insert("serde_json");
            }
        }

        imports
    }

    pub fn generate_tests(
        &self,
        types: &[TypeInfo],
        source_file: &std::path::Path,
    ) -> Result<String> {
        if types.is_empty() {
            return Err(TypeTesterError::GenerationError(
                "No types found to generate tests for".to_string(),
            ));
        }

        let mut test_stream = TokenStream::new();

        let source_file_name = source_file
            .file_name()
            .ok_or_else(|| {
                TypeTesterError::GenerationError("Could not get source file name".to_string())
            })?
            .to_string_lossy();

        let source_path = format!("../{}", source_file_name);
        test_stream.extend(quote! {
            #[path = #source_path]
            mod parent;
        });

        let type_names: Vec<_> = types.iter().map(|t| format_ident!("{}", &t.name)).collect();

        let mut imports = self.collect_required_imports(types);
        imports.remove("std::default::Default");

        let import_vec: Vec<_> = imports
            .into_iter()
            .filter(|i| !i.is_empty())
            .map(|i| {
                let segments: Vec<_> = i.split("::").map(|s| format_ident!("{}", s)).collect();
                quote!(#(#segments)::*)
            })
            .collect();

        let mut used_imports = TokenStream::new();
        if !type_names.is_empty() {
            used_imports.extend(quote! {
                use parent::{#(#type_names),*};
            });
        }
        if !import_vec.is_empty() {
            used_imports.extend(quote! {
                #(use #import_vec;)*
            });
        }

        test_stream.extend(used_imports);

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

            test_stream.extend(quote! {
                #[cfg(test)]
                mod #test_name {
                    use super::*;
                    #type_tests
                }
            });
        }

        Ok(test_stream.to_string())
    }
}
