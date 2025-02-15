use crate::analyzer::TypeInfo;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::TestGenerator;

pub struct SizeTestGenerator;

impl SizeTestGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TestGenerator for SizeTestGenerator {
    fn is_applicable(&self, _type_info: &TypeInfo) -> bool {
        true // Size tests are applicable to all types
    }

    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);

        quote! {
            #[test]
            fn check_size_and_alignment() {
                let size = std::mem::size_of::<#type_name>();
                let align = std::mem::align_of::<#type_name>();
                
                assert!(size > 0, "Type should have non-zero size");
                assert!(align > 0, "Type should have non-zero alignment");
                assert!(size % align == 0, "Size should be a multiple of alignment");
                
                println!("Type {} metrics:", stringify!(#type_name));
                println!("  - Size: {} bytes", size);
                println!("  - Alignment: {} bytes", align);
            }

            #[test]
            fn check_default_instance_size() {
                let instance = #type_name::default();
                let instance_size = std::mem::size_of_val(&instance);
                let type_size = std::mem::size_of::<#type_name>();
                
                assert_eq!(instance_size, type_size, 
                    "Instance size should match type size");
            }

            #[test]
            fn check_option_size() {
                let option_size = std::mem::size_of::<Option<#type_name>>();
                let type_size = std::mem::size_of::<#type_name>();
                
                println!("Option<{}> size: {} bytes", stringify!(#type_name), option_size);
                println!("Raw {} size: {} bytes", stringify!(#type_name), type_size);
            }
        }
    }
} 