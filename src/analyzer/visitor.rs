use crate::analyzer::{FieldInfo, TypeInfo, TypeKind};
use crate::error::Result;
use quote::ToTokens;
use syn::{Attribute, Field, File, Item, ItemEnum, ItemImpl, ItemStruct};

pub struct TypeVisitor;

impl TypeVisitor {
    pub fn new() -> Self {
        Self
    }

    pub fn visit_file(&self, file: &File) -> Result<Vec<TypeInfo>> {
        let mut types = Vec::new();
        let mut impls = Vec::new();

        for item in &file.items {
            match item {
                Item::Struct(s) => {
                    types.push((s.ident.to_string(), self.visit_struct(s)));
                }
                Item::Enum(e) => {
                    types.push((e.ident.to_string(), self.visit_enum(e)));
                }
                Item::Impl(i) => {
                    if let Some(name) = self.extract_impl_type_name(i) {
                        impls.push((name, self.extract_impl_trait_name(i)));
                    }
                }
                _ => continue,
            }
        }

        let types = types
            .into_iter()
            .map(|(name, mut type_info)| {
                type_info.manual_impls = impls
                    .iter()
                    .filter(|(impl_name, _)| impl_name == &name)
                    .filter_map(|(_, trait_name)| trait_name.clone())
                    .collect();
                type_info
            })
            .collect();

        Ok(types)
    }

    fn visit_struct(&self, item: &ItemStruct) -> TypeInfo {
        TypeInfo {
            name: item.ident.to_string(),
            kind: TypeKind::Struct,
            derives: self.extract_derives(&item.attrs),
            attributes: self.extract_attributes(&item.attrs),
            fields: self.extract_fields(&item.fields),
            manual_impls: Vec::new(), // Will be filled in second pass
        }
    }

    fn visit_enum(&self, item: &ItemEnum) -> TypeInfo {
        TypeInfo {
            name: item.ident.to_string(),
            kind: TypeKind::Enum,
            derives: self.extract_derives(&item.attrs),
            attributes: self.extract_attributes(&item.attrs),
            fields: Vec::new(),
            manual_impls: Vec::new(), // Will be filled in second pass
        }
    }

    fn extract_impl_type_name(&self, item: &ItemImpl) -> Option<String> {
        if let Some((_, path, _)) = &item.trait_ {
            item.self_ty.to_token_stream().to_string().into()
        } else {
            item.self_ty.to_token_stream().to_string().into()
        }
    }

    fn extract_impl_trait_name(&self, item: &ItemImpl) -> Option<String> {
        item.trait_.as_ref().map(|(_, path, _)| {
            path.segments
                .last()
                .map(|seg| seg.ident.to_string())
                .unwrap_or_default()
        })
    }

    fn extract_fields(&self, fields: &syn::Fields) -> Vec<FieldInfo> {
        fields
            .iter()
            .map(|field| self.field_to_info(field))
            .collect()
    }

    fn field_to_info(&self, field: &Field) -> FieldInfo {
        FieldInfo {
            name: field
                .ident
                .as_ref()
                .map(|i| i.to_string())
                .unwrap_or_default(),
            ty: field.ty.to_token_stream().to_string(),
            attributes: self.extract_attributes(&field.attrs),
        }
    }

    fn extract_derives(&self, attrs: &[Attribute]) -> Vec<String> {
        attrs
            .iter()
            .filter_map(|attr| {
                if !attr.path().is_ident("derive") {
                    return None;
                }

                let meta = attr.meta.require_list().ok()?;
                Some(
                    meta.tokens
                        .to_string()
                        .trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<_>>(),
                )
            })
            .flatten()
            .collect()
    }

    fn extract_attributes(&self, attrs: &[Attribute]) -> Vec<String> {
        attrs
            .iter()
            .map(|attr| attr.to_token_stream().to_string())
            .collect()
    }
}
