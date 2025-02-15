use crate::error::Result;
use crate::analyzer::{TypeInfo, TypeKind};
use syn::{File, Item, ItemStruct, ItemEnum, Attribute};
use quote::ToTokens;

pub struct TypeVisitor;

impl TypeVisitor {
    pub fn new() -> Self {
        Self
    }

    pub fn visit_file(&self, file: &File) -> Result<Vec<TypeInfo>> {
        let mut types = Vec::new();

        for item in &file.items {
            match item {
                Item::Struct(s) => types.push(self.visit_struct(s)),
                Item::Enum(e) => types.push(self.visit_enum(e)),
                _ => continue,
            }
        }

        Ok(types)
    }

    fn visit_struct(&self, item: &ItemStruct) -> TypeInfo {
        TypeInfo {
            name: item.ident.to_string(),
            kind: TypeKind::Struct,
            derives: self.extract_derives(&item.attrs),
            attributes: self.extract_attributes(&item.attrs),
        }
    }

    fn visit_enum(&self, item: &ItemEnum) -> TypeInfo {
        TypeInfo {
            name: item.ident.to_string(),
            kind: TypeKind::Enum,
            derives: self.extract_derives(&item.attrs),
            attributes: self.extract_attributes(&item.attrs),
        }
    }

    fn extract_derives(&self, attrs: &[Attribute]) -> Vec<String> {
        attrs.iter()
            .filter_map(|attr| {
                if !attr.path().is_ident("derive") {
                    return None;
                }

                let meta = attr.meta.require_list().ok()?;
                Some(meta.tokens.to_string()
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .collect::<Vec<_>>())
            })
            .flatten()
            .collect()
    }

    fn extract_attributes(&self, attrs: &[Attribute]) -> Vec<String> {
        attrs.iter()
            .map(|attr| attr.to_token_stream().to_string())
            .collect()
    }
} 