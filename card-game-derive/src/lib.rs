extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse::Parse, parse_macro_input};

#[proc_macro_derive(Filter)]
pub fn filter(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote::quote! {
        impl #impl_generics card_game::identifications::FilterSupertype<Self> for #name #ty_generics #where_clause {}
    }
    .into()
}
