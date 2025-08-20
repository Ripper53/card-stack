extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{
    Data, DeriveInput, GenericArgument, Ident, PathArguments, Type, parse_macro_input,
    spanned::Spanned,
};

#[proc_macro_derive(SuperCommand)]
pub fn super_command(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let (commands, variant_names) = match &ast.data {
        Data::Enum(s) => {
            let mut variant_names = Vec::with_capacity(s.variants.len());
            let mut commands = Vec::with_capacity(s.variants.len());
            //let mut in_states = Vec::new();
            //let mut out_states = Vec::new();
            for (variant, ty) in s.variants.iter().map(|v| {
                (
                    &v.ident,
                    &v.fields.iter().next().expect("Expected command field").ty,
                )
            }) {
                variant_names.push(variant);
                commands.push(ty);
            }
            //(in_states, out_states)
            (commands, variant_names)
        }
        Data::Struct(_) => panic!(),
        Data::Union(_) => panic!(),
    };
    let in_states_ident = Ident::new(&format!("{}InStates", name), ast.span());
    let out_states_ident = Ident::new(&format!("{}OutStates", name), ast.span());
    quote::quote! {
        pub enum #in_states_ident #ty_generics #where_clause {
            #(
                #variant_names(<#commands as ::card_game::commands::Command>::InState),
            )*
        }
        pub enum #out_states_ident #ty_generics #where_clause {
            #(
                #variant_names(<#commands as ::card_game::commands::Command>::OutState),
            )*
        }
        #(
            impl #impl_generics ::std::convert::From<#commands> for #name #ty_generics {
                fn from(value: #commands) -> Self {
                    Self::#variant_names(value)
                }
            }
        )*
        impl #impl_generics ::card_game::commands::Command for #name #ty_generics #where_clause {
            type Data = Self;
            type InState = #in_states_ident #ty_generics;
            type OutState = #out_states_ident #ty_generics;
            fn new(super_command: Self) -> Self {
                super_command
            }
            fn execute(&mut self, state: Self::InState) -> Self::OutState {
                match self {
                    #(
                        Self::#variant_names(command) => if let #in_states_ident::#variant_names(state) = state {
                            #out_states_ident::#variant_names(command.execute(state))
                        } else {
                            panic!()
                        },
                    )*
                }
            }
            fn undo(self, state: Self::OutState) -> Self::InState {
                match self {
                    #(
                        Self::#variant_names(command) => if let #out_states_ident::#variant_names(state) = state {
                            #in_states_ident::#variant_names(command.undo(state))
                        } else {
                            panic!()
                        },
                    )*
                }
            }
        }
    }
    .into()
}
