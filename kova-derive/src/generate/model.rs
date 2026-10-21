use crate::generate::from_row::expand_from_row_impl;
use crate::generate::unique_field_types;
use crate::parse::StructDef;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn expand(def: &StructDef) -> TokenStream {
    let vis = &def.vis;
    let struct_ident = &def.ident;
    let table_name = &def.table_name;
    let from_row_impl = expand_from_row_impl(def);

    // Only non-auto fields are encoded in `to_values()`, so `#[kova(auto)]` fields
    // don't need to force `for<'a> &'a Ty: Encode<'a, X>` bounds on `Model`.
    let encode_tys = unique_field_types(def.fields.iter().filter(|f| !f.is_auto));

    let mut const_columns = Vec::with_capacity(def.fields.len());
    let mut meta_columns = Vec::with_capacity(def.fields.len());
    let mut insert_values = Vec::with_capacity(def.fields.len());

    for f in &def.fields {
        let field_ident = &f.ident;
        let field_ty = &f.ty;
        let col_name = &f.col_name;
        let is_auto = f.is_auto;

        const_columns.push(quote! {
            pub const #field_ident: ::kova::dsl::Column<#field_ty> =
                ::kova::dsl::Column::new(#table_name, #col_name, #is_auto);
        });

        meta_columns.push(quote! {
            ::kova::types::ColumnMeta::new(#table_name, #col_name, #is_auto)
        });

        if !is_auto {
            insert_values.push(quote! {
                (#col_name, ::kova::codec::Encode::<'a, X>::encode(&self.#field_ident)?)
            });
        }
    }

    quote! {
        #[allow(non_upper_case_globals)]
        #vis const #struct_ident: ::kova::dsl::Table<#struct_ident> =
            ::kova::dsl::Table::new(#table_name, #table_name);

        #[allow(non_upper_case_globals)]
        impl #struct_ident {
            #(#const_columns)*
        }

        impl ::kova::types::Relation for #struct_ident {
            const NAME: &'static ::core::primitive::str = #table_name;
        }

        #from_row_impl

        impl<X> ::kova::types::Model<::kova::types::Engine, X> for #struct_ident
            where
                Self: ::kova::types::FromRow<::kova::types::Engine, X>,
                #( for<'a> &'a #encode_tys: ::kova::codec::Encode<'a, X>, )* {
            const COLUMNS_META: &'static [::kova::types::ColumnMeta] = &[
                #(#meta_columns),*
            ];

            fn to_values<'a>(
                &'a self,
            ) -> ::kova::Result<
                ::std::vec::Vec<(
                    &'static ::core::primitive::str,
                    <::kova::types::Engine as ::kova::types::Database>::Value<'a>,
                )>,
            > {
                ::kova::Result::Ok(::std::vec![
                    #(#insert_values),*
                ])
            }
        }
    }
}
