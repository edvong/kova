use crate::ModelDef;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn generate_from_row_code(model: &ModelDef) -> TokenStream {
    let struct_ident = &model.ident;
    let table_name = &model.table_name;

    let mut row_reads = Vec::new();
    let mut values_reads = Vec::new();

    for field in &model.fields {
        let field_ident = &field.ident;
        let field_ty = &field.ty;
        let col_name = &field.column_name;
        let is_json = field.is_json;

        if field.is_ignored {
            row_reads.push(quote! { #field_ident: ::std::default::Default::default() });
            continue;
        }

        let strict_col_name = format!("{}.{}", table_name, col_name);

        let parse_logic = if is_json {
            quote! {
                <::kova::types::Json<#field_ty> as ::kova::types::FromValue>::from_value(val)
                    .map_err(|e| ::kova::Error::ParseError(format!("Column '{}': {}", stringify!(#field_ident), e)))?
                    .0
            }
        } else {
            quote! {
                <#field_ty as ::kova::types::FromValue>::from_value(val)
                    .map_err(|e| ::kova::Error::ParseError(format!("Column '{}': {}", stringify!(#field_ident), e)))?
            }
        };

        let get_val_expr = quote! {
            row.get_value(#strict_col_name).or_else(|_| row.get_value(#col_name))
        };

        row_reads.push(quote! {
            #field_ident: {
                let val = #get_val_expr.map_err(|e| ::kova::Error::ParseError(e.to_string()))?;
                #parse_logic
            }
        });

        values_reads.push(quote! {
            #get_val_expr.map_err(|e| ::kova::Error::ParseError(e.to_string()))?
        });
    }

    quote! {
        impl ::kova::types::FromRow for #struct_ident {
            type DB = ::kova::types::Engine;

            fn from_row(row: &<Self::DB as ::kova::types::Database>::Row<'_>) -> ::kova::Result<Self> {
                ::kova::Result::Ok(Self {
                    #(#row_reads),*
                })
            }

            fn from_row_as_values<'a>(row: &'a <Self::DB as ::kova::types::Database>::Row<'_>) -> ::kova::Result<::std::vec::Vec<<Self::DB as ::kova::types::Database>::Value<'a>>> {
                ::kova::Result::Ok(::std::vec![ #(#values_reads),* ])
            }
        }
    }
}
