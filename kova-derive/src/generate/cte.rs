use crate::generate::expand_identifier_columns;
use crate::parse::StructDef;
use proc_macro2::TokenStream;
use quote::quote;

pub(crate) fn expand(def: &StructDef) -> TokenStream {
    let struct_ident = &def.ident;
    let table_name = &def.table_name;
    let columns_block = expand_identifier_columns(def);

    quote! {
        #columns_block

        impl ::kova::types::Relation for #struct_ident {
            const NAME: &'static ::core::primitive::str = #table_name;
        }

        impl ::kova::types::Cte for #struct_ident {}
    }
}
