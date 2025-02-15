use super::TestGenerator;
use crate::analyzer::TypeInfo;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct DeriveTestGenerator;

impl DeriveTestGenerator {
    pub fn new() -> Self {
        Self
    }

    fn generate_debug_test(
        &self,
        type_name: &proc_macro2::Ident,
        has_default: bool,
    ) -> TokenStream {
        if has_default {
            quote! {
                #[test]
                fn implements_debug() {
                    let instance = #type_name::default();
                    let _result = format!("{:?}", instance);
                }
            }
        } else {
            quote! {
                #[test]
                fn implements_debug() {
                    let instance = unsafe { std::mem::zeroed::<#type_name>() };
                    let _result = format!("{:?}", instance);
                }
            }
        }
    }

    fn generate_clone_test(
        &self,
        type_name: &proc_macro2::Ident,
        has_default: bool,
    ) -> TokenStream {
        if has_default {
            quote! {
                #[test]
                fn implements_clone() {
                    let original = #type_name::default();
                    let cloned = original.clone();
                    assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
                }
            }
        } else {
            quote! {
                #[test]
                fn implements_clone() {
                    let original = unsafe { std::mem::zeroed::<#type_name>() };
                    let cloned = original.clone();
                    assert!(std::mem::size_of_val(&original) == std::mem::size_of_val(&cloned));
                }
            }
        }
    }

    fn generate_partialeq_test(
        &self,
        type_name: &proc_macro2::Ident,
        has_default: bool,
    ) -> TokenStream {
        if has_default {
            quote! {
                #[test]
                fn implements_partialeq() {
                    let a = #type_name::default();
                    let b = #type_name::default();
                    assert_eq!(a, b);
                }
            }
        } else {
            quote! {
                #[test]
                fn implements_partialeq() {
                    let a = unsafe { std::mem::zeroed::<#type_name>() };
                    let b = unsafe { std::mem::zeroed::<#type_name>() };
                    assert_eq!(a, b);
                }
            }
        }
    }

    fn generate_serde_test(
        &self,
        type_name: &proc_macro2::Ident,
        has_default: bool,
    ) -> TokenStream {
        if has_default {
            quote! {
                #[test]
                fn implements_serde_traits() {
                    let instance = #type_name::default();
                    let serialized = serde_json::to_string(&instance).unwrap();
                    let _deserialized: #type_name = serde_json::from_str(&serialized).unwrap();
                }
            }
        } else {
            quote! {
                #[test]
                fn implements_serde_traits() {
                    let _serialize_fn: fn(&#type_name) -> Result<String, serde_json::Error> = serde_json::to_string;
                    let _deserialize_fn: fn(&str) -> Result<#type_name, serde_json::Error> = serde_json::from_str;
                }
            }
        }
    }
}

impl TestGenerator for DeriveTestGenerator {
    fn is_applicable(&self, type_info: &TypeInfo) -> bool {
        !type_info.derives.is_empty() || !type_info.manual_impls.is_empty()
    }

    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);
        let mut tests = TokenStream::new();

        let has_default = type_info.has_implementation("Default");

        if type_info.has_implementation("Debug") {
            tests.extend(self.generate_debug_test(&type_name, has_default));
        }

        if type_info.has_implementation("Clone") {
            tests.extend(self.generate_clone_test(&type_name, has_default));
        }

        if type_info.has_implementation("PartialEq") {
            tests.extend(self.generate_partialeq_test(&type_name, has_default));
        }

        if type_info.has_implementation("Serialize") && type_info.has_implementation("Deserialize")
        {
            tests.extend(self.generate_serde_test(&type_name, has_default));
        }

        tests
    }

    fn required_imports(&self) -> Vec<&'static str> {
        vec!["std::mem"]
    }
}
