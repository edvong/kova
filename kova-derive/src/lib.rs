extern crate proc_macro;

mod generate;
mod parse;

use parse::{DeriveRole, parse_struct};
use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Model, attributes(kova))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match parse_struct(&ast, DeriveRole::Model) {
        Ok(def) => generate::model::expand(&def).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

// Keep `attributes(kova)` registered on `FromRow` and `Cte` even though they don't
// use `#[kova(...)]`. Otherwise rustc intercepts `#[kova(auto)]` with a generic "unknown
// attribute" error before `parse_field` can explain that `#[kova(auto)]` is Model-only.
#[proc_macro_derive(FromRow, attributes(kova))]
pub fn derive_from_row(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match parse_struct(&ast, DeriveRole::FromRow) {
        Ok(def) => generate::from_row::expand(&def).into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro_derive(Cte, attributes(kova))]
pub fn derive_cte(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match parse_struct(&ast, DeriveRole::Cte) {
        Ok(def) => generate::cte::expand(&def).into(),
        Err(err) => err.to_compile_error().into(),
    }
}