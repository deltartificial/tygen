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
    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);

        quote! {
            #[test]
            fn check_size() {
                let size = std::mem::size_of::<#type_name>();
                assert!(size > 0, "Type should have non-zero size");
                println!("Size of {} is {} bytes", stringify!(#type_name), size);
            }

            #[test]
            fn check_alignment() {
                let align = std::mem::align_of::<#type_name>();
                assert!(align > 0, "Type should have non-zero alignment");
                println!("Alignment of {} is {} bytes", stringify!(#type_name), align);
            }
        }
    }
} 