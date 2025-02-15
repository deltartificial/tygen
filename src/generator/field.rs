use super::TestGenerator;
use crate::analyzer::{TypeInfo, TypeKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct FieldTestGenerator;

impl FieldTestGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TestGenerator for FieldTestGenerator {
    fn is_applicable(&self, type_info: &TypeInfo) -> bool {
        type_info.kind == TypeKind::Struct && !type_info.fields.is_empty()
    }

    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);
        let mut tests = TokenStream::new();

        // Test field accessibility
        for field in &type_info.fields {
            let field_name = format_ident!("{}", field.name);
            let test_name = format_ident!("test_field_{}", field.name);
            let field_type = field
                .ty
                .parse::<proc_macro2::TokenStream>()
                .unwrap_or_else(|_| quote!(()));

            tests.extend(quote! {
                #[test]
                fn #test_name() {
                    let instance = #type_name::default();
                    let _: #field_type = instance.#field_name;
                }
            });
        }

        // Test field attributes
        tests.extend(quote! {
            #[test]
            fn check_field_attributes() {
                let instance = #type_name::default();
                let fields = std::mem::size_of_val(&instance);
                assert!(fields > 0, "Type should have valid fields");
            }
        });

        tests
    }
}
