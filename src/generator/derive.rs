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
    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);
        let derives = &type_info.derives;
        let mut tests = TokenStream::new();

        if derives.contains(&"Debug".to_string()) {
            tests.extend(quote! {
                #[test]
                fn implements_debug() {
                    let _result = format!("{:?}", #type_name::default());
                }
            });
        }

        if derives.contains(&"Clone".to_string()) {
            tests.extend(quote! {
                #[test]
                fn implements_clone() {
                    let original = #type_name::default();
                    let _cloned = original.clone();
                }
            });
        }

        if derives.contains(&"PartialEq".to_string()) {
            tests.extend(quote! {
                #[test]
                fn implements_partial_eq() {
                    let a = #type_name::default();
                    let b = #type_name::default();
                    assert_eq!(a, b);
                }
            });
        }

        tests
    }
} 