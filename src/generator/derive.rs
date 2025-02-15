use crate::analyzer::TypeInfo;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::TestGenerator;

pub struct DeriveTestGenerator;

impl DeriveTestGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TestGenerator for DeriveTestGenerator {
    fn is_applicable(&self, type_info: &TypeInfo) -> bool {
        !type_info.derives.is_empty()
    }

    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);
        let derives = &type_info.derives;
        let mut tests = TokenStream::new();

        // Common derives
        for derive in ["Debug", "Clone", "PartialEq"] {
            if derives.contains(&derive.to_string()) {
                let test_name = format_ident!("implements_{}", derive.to_lowercase());
                let test = match derive {
                    "Debug" => quote! {
                        #[test]
                        fn #test_name() {
                            let instance = #type_name::default();
                            let _result = format!("{:?}", instance);
                        }
                    },
                    "Clone" => quote! {
                        #[test]
                        fn #test_name() {
                            let original = #type_name::default();
                            let cloned = original.clone();
                            assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
                        }
                    },
                    "PartialEq" => quote! {
                        #[test]
                        fn #test_name() {
                            let a = #type_name::default();
                            let b = #type_name::default();
                            assert_eq!(a, b);
                            assert!(a == b);
                        }
                    },
                    _ => continue,
                };
                tests.extend(test);
            }
        }

        // Serde derives
        if derives.contains(&"Serialize".to_string()) && derives.contains(&"Deserialize".to_string()) {
            tests.extend(quote! {
                #[test]
                fn implements_serde_traits() {
                    let _: fn(&#type_name) -> Result<String, _> = serde_json::to_string;
                    let _: fn(&str) -> Result<#type_name, _> = serde_json::from_str;
                }
            });
        }

        tests
    }
} 