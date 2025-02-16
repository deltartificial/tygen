use super::TestGenerator;
use crate::analyzer::TypeInfo;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

pub struct SerializationTestGenerator;

impl SerializationTestGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl TestGenerator for SerializationTestGenerator {
    fn is_applicable(&self, type_info: &TypeInfo) -> bool {
        type_info.requires_serde()
    }

    fn required_imports(&self) -> Vec<&'static str> {
        vec!["serde_json"]
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

            #[test]
            fn serialization_error_handling() {
                let invalid_json = r#"{"invalid": json"#;
                let result = serde_json::from_str::<#type_name>(invalid_json);
                assert!(result.is_err(), "Should fail with invalid JSON");
            }
        }
    }

    fn generator_type(&self) -> &'static str {
        "serialization"
    }
}
