mod derive;
mod field;
mod serialization;
mod size;
mod suite;

use proc_macro2::TokenStream;

pub use self::derive::DeriveTestGenerator;
pub use self::field::FieldTestGenerator;
pub use self::serialization::SerializationTestGenerator;
pub use self::size::SizeTestGenerator;
pub use self::suite::TestSuite;

#[derive(Default)]
pub struct TestConfig {
    pub check_derives: bool,
    pub check_serialization: bool,
    pub check_size: bool,
    pub check_fields: bool,
}

pub trait TestGenerator {
    fn generate(&self, type_info: &crate::analyzer::TypeInfo) -> TokenStream;
    fn is_applicable(&self, type_info: &crate::analyzer::TypeInfo) -> bool;
    fn required_imports(&self) -> Vec<&'static str> {
        Vec::new()
    }
    fn generator_type(&self) -> &'static str {
        "default"
    }
}
