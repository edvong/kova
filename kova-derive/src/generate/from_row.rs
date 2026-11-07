use crate::generate::{expand_identifier_columns, unique_field_types};
use crate::parse::StructDef;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn expand(def: &StructDef) -> TokenStream {
    let columns_block = expand_identifier_columns(def, "");
    let from_row_impl = expand_from_row_impl(def);

    quote! {
        #columns_block
        #from_row_impl
    }
}

// Reused by `#[derive(Model)]`. Every field (including `#[kova(auto)]`) is decoded when reading rows.
pub(crate) fn expand_from_row_impl(def: &StructDef) -> TokenStream {
    let struct_ident = &def.ident;
    let col_count = def.fields.len();
    let decode_tys = unique_field_types(&def.fields);

    let field_reads = def.fields.iter().map(|f| {
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        let col_name = &f.col_name;

        quote! {
            #field_ident: ::kova::types::Row::get_column::<#field_ty, X>(
                row,
                idx,
                #col_name,
            )?
        }
    });

    quote! {
        impl<X> ::kova::types::FromRow<::kova::types::Engine, X> for #struct_ident
            where
                #( #decode_tys: ::kova::codec::Decode<X>, )* {
            const COLUMN_COUNT: ::core::primitive::usize = #col_count;

            #[inline(always)]
            fn from_row_at(
                row: &<::kova::types::Engine as ::kova::types::Database>::Row<'_>,
                idx: &mut ::core::primitive::usize,
            ) -> ::kova::Result<Self> {
                ::kova::Result::Ok(Self {
                    #(#field_reads),*
                })
            }
        }
    }
}