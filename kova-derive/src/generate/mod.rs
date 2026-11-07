pub mod cte;
pub mod from_row;
pub mod model;

use crate::parse::{FieldDef, StructDef};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashSet;
use syn::Type;

// Deduplicates field types (preserving order) so generated `where` clauses don't repeat bounds.
pub(crate) fn unique_field_types<'a>(
    fields: impl IntoIterator<Item = &'a FieldDef>,
) -> Vec<&'a Type> {
    let mut seen = HashSet::new();
    let mut unique = Vec::new();
    for field in fields {
        let ty = &field.ty;
        let key = quote!(#ty).to_string();
        if seen.insert(key) {
            unique.push(ty);
        }
    }
    unique
}

// Shared by `#[derive(FromRow)]` and `#[derive(Cte)]`: generates read-only `IdentifierMeta` columns.
pub(crate) fn expand_identifier_columns(def: &StructDef, source: &str) -> TokenStream {
    let struct_ident = &def.ident;

    let const_cols = def.fields.iter().map(|f| {
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        let col_name = &f.col_name;
        quote! {
            pub const #field_ident: ::kova::dsl::Column<#field_ty, ::kova::types::IdentifierMeta> =
                ::kova::dsl::Column::identifier(#source, #col_name);
        }
    });

    quote! {
        #[allow(non_upper_case_globals)]
        impl #struct_ident {
            #(#const_cols)*
        }
    }
}
