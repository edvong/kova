use crate::ModelDef;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_model_code(model: &ModelDef) -> TokenStream {
    let struct_ident = &model.ident;
    let table_name = &model.table_name;

    let mut const_columns = Vec::new();
    let mut meta_columns = Vec::new();
    let mut insert_values = Vec::new();

    for field in &model.fields {
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        let col_name = &field.column_name;
        let is_auto = field.is_autoincrement;
        let is_json = field.is_json;
        let is_ignore = field.is_ignored;

        meta_columns.push(quote! {
            ::kova::types::ColumnMeta {
                table_name: #table_name,
                column_name: #col_name,
                is_autoincrement: #is_auto,
                is_ignore: #is_ignore,
            }
        });

        let column_generic_type = if is_json {
            quote! { ::kova::types::Json<#field_ty> }
        } else {
            quote! { #field_ty }
        };

        let const_name = quote::format_ident!("{}", field_ident.to_string().to_lowercase());
        const_columns.push(quote! {
            #[allow(non_upper_case_globals)]
            pub const #const_name: ::kova::dsl::Column<#column_generic_type> =
                ::kova::dsl::Column::new(
                    ::kova::types::ColumnMeta {
                        table_name: #table_name,
                        column_name: #col_name,
                        is_autoincrement: #is_auto,
                        is_ignore: #is_ignore,
                    }
                );
        });

        if !is_ignore && !is_auto {
            let value_logic = if is_json {
                quote! { ::kova::types::ToValue::to_value(&::kova::types::Json(&self.#field_ident))? }
            } else {
                quote! { ::kova::types::ToValue::to_value(&self.#field_ident)? }
            };

            insert_values.push(quote! { (#col_name, #value_logic) });
        }
    }

    quote! {
        impl #struct_ident {
            #(#const_columns)*
        }

        impl ::kova::types::Model for #struct_ident {
            const TABLE_NAME: &'static str = #table_name;
            const COLUMNS_META: &'static [::kova::types::ColumnMeta] = &[ #(#meta_columns),* ];

            fn to_values<'a>(&'a self) -> ::kova::Result<::std::vec::Vec<(&'static str, <Self::DB as ::kova::types::Database>::Value<'a>)>> {
                ::kova::Result::Ok( ::std::vec![ #(#insert_values),* ] )
            }
        }
    }
}
