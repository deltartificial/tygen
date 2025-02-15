use crate::analyzer::TypeInfo;
use proc_macro2::TokenStream;
use quote::{quote, format_ident};
use super::TestGenerator;

pub struct SerializationTestGenerator;

impl SerializationTestGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TestGenerator for SerializationTestGenerator {
    fn is_applicable(&self, type_info: &TypeInfo) -> bool {
        type_info.derives.contains(&"Serialize".to_string()) &&
        type_info.derives.contains(&"Deserialize".to_string())
    }

    fn generate(&self, type_info: &TypeInfo) -> TokenStream {
        let type_name = format_ident!("{}", type_info.name);

        quote! {
            #[test]
            fn serialization_roundtrip() {
                let original = #type_name::default();
                let serialized = serde_json::to_string(&original)
                    .expect("Failed to serialize");
                let deserialized: #type_name = serde_json::from_str(&serialized)
                    .expect("Failed to deserialize");
                assert_eq!(original, deserialized);
            }

            #[test]
            fn serialization_pretty_format() {
                let value = #type_name::default();
                let serialized = serde_json::to_string_pretty(&value)
                    .expect("Failed to serialize with pretty format");
                let deserialized: #type_name = serde_json::from_str(&serialized)
                    .expect("Failed to deserialize from pretty format");
                assert_eq!(value, deserialized);
            }

            #[test]
            fn serialization_json_schema() {
                let value = #type_name::default();
                let serialized = serde_json::to_value(&value)
                    .expect("Failed to convert to JSON value");
                assert!(serialized.is_object() || serialized.is_array(), 
                    "Serialized value must be a JSON object or array");
            }
        }
    }
} 