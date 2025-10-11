extern crate proc_macro;

use darling::FromDeriveInput;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::format_ident;
use syn::{
    Data, DeriveInput, Expr, Ident, Index, LitInt, Type, parse::Parse, parse_macro_input,
    spanned::Spanned, token::Comma,
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

#[derive(darling::FromDeriveInput)]
#[darling(attributes(state_filter_input))]
struct StateFilterInputData {
    remainder_type: Option<Type>,
    remainder: Option<Expr>,
}

#[proc_macro_derive(StateFilterInput, attributes(state_filter_input))]
pub fn state_filter_input(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let data = StateFilterInputData::from_derive_input(&ast).unwrap();
    let remainder_code = if let Some(remainder_type) = data.remainder_type {
        let remainder_expr = data.remainder.expect("expected `remainder` expression");
        quote::quote! {
            impl #impl_generics card_game::validation::StateFilterInputConversion<Self> for #name #ty_generics #where_clause {
                type Remainder = #remainder_type;
                fn split_take(self) -> (Self, Self::Remainder) {
                    (self, #remainder_expr)
                }
            }
        }
    } else {
        quote::quote! {
            impl #impl_generics card_game::validation::StateFilterInputConversion<Self> for #name #ty_generics #where_clause {
                type Remainder = ();
                fn split_take(self) -> (Self, Self::Remainder) {
                    (self, ())
                }
            }
        }
    };
    let mut generics_1 = ast.generics.clone();
    generics_1
        .params
        .push(syn::GenericParam::Type(syn::TypeParam {
            attrs: Vec::new(),
            ident: Ident::new("T", Span::call_site()),
            colon_token: None,
            bounds: syn::punctuated::Punctuated::new(),
            eq_token: None,
            default: None,
        }));
    let (_impl_generics_1, _ty_generics_1, _where_clause_1) = generics_1.split_for_impl();
    let mut generics_2 = ast.generics.clone();
    generics_2
        .params
        .push(syn::GenericParam::Type(syn::TypeParam {
            attrs: Vec::new(),
            ident: Ident::new("T0", Span::call_site()),
            colon_token: None,
            bounds: syn::punctuated::Punctuated::new(),
            eq_token: None,
            default: None,
        }));
    generics_2
        .params
        .push(syn::GenericParam::Type(syn::TypeParam {
            attrs: Vec::new(),
            ident: Ident::new("T1", Span::call_site()),
            colon_token: None,
            bounds: syn::punctuated::Punctuated::new(),
            eq_token: None,
            default: None,
        }));
    let (_impl_generics_2, _ty_generics_2, _where_clause_2) = generics_2.split_for_impl();
    quote::quote! {
        impl #impl_generics card_game::validation::StateFilterInput for #name #ty_generics #where_clause {}
        #remainder_code
        /*impl #impl_generics_1 card_game::validation::StateFilterInputConversion<#name #ty_generics> for (#name #ty_generics, T) #where_clause {
            type Remainder = (T,);
            fn combine(input: #name #ty_generics, remainder: Self::Remainder) -> Self {
                (input, remainder.0)
            }
            fn split_take(self) -> (#name #ty_generics, Self::Remainder) {
                (self.0, (self.1,))
            }
        }
        impl #impl_generics_2 card_game::validation::StateFilterInputConversion<#name #ty_generics> for (#name #ty_generics, T0, T1) #where_clause {
            type Remainder = (T0, T1);
            fn combine(input: #name #ty_generics, remainder: Self::Remainder) -> Self {
                (input, remainder.0, remainder.1)
            }
            fn split_take(self) -> (#name #ty_generics, Self::Remainder) {
                (self.0, (self.1, self.2))
            }
        }
        impl #impl_generics_2 card_game::validation::StateFilterInputConversion<(#name #ty_generics, T0)> for (#name #ty_generics, T0, T1) #where_clause {
            type Remainder = (T1,);
            fn combine(input: (#name #ty_generics, T0), remainder: Self::Remainder) -> Self {
                (input.0, input.1, remainder.0)
            }
            fn split_take(self) -> ((#name #ty_generics, T0), Self::Remainder) {
                ((self.0, self.1), (self.2,))
            }
        }*/
        /*impl #impl_generics_1 card_game::validation::StateFilterInputConversion<T> for (#name #ty_generics, T) #where_clause {
            type Remainder = (#name #ty_generics,);
            fn combine(input: T, remainder: Self::Remainder) -> Self {
                (remainder.0, input)
            }
            fn split_take(self) -> (T, Self::Remainder) {
                (self.0, (self.1,))
            }
        }*/
        /*impl #impl_generics_2 card_game::validation::StateFilterInputConversion<(#name #ty_generics, T0)> for (#name #ty_generics, T0, T1) #where_clause {
            type Remainder = (T1,);
            fn combine(input: (#name #ty_generics, T0), remainder: Self::Remainder) -> Self {
                (input.0, input.1, remainder.0)
            }
            fn split_take(self) -> ((#name #ty_generics, T0), Self::Remainder) {
                ((self.0, self.1), (self.2,))
            }
        }*/
        /*impl #impl_generics_2 card_game::validation::StateFilterInputConversion<(T0, T1)> for (T0, #name #ty_generics, T1) #where_clause {
            type Remainder = (#name #ty_generics,);
            fn combine(input: (T0, T1), remainder: Self::Remainder) -> Self {
                (input.0, remainder.0, input.1)
            }
            fn split_take(self) -> ((T0, T1), Self::Remainder) {
                ((self.0, self.2), (self.1,))
            }
        }
        impl #impl_generics_2 card_game::validation::StateFilterInputConversion<(T0, T1)> for (#name #ty_generics, T0, T1) #where_clause {
            type Remainder = (#name #ty_generics,);
            fn combine(input: (T0, T1), remainder: Self::Remainder) -> Self {
                (remainder.0, input.0, input.1)
            }
            fn split_take(self) -> ((T0, T1), Self::Remainder) {
                ((self.1, self.2), (self.0,))
            }
        }*/
    }
    .into()
}

struct StateFilterInputs {
    input_name: Ident,
    all_types: Vec<Type>,
}

impl Parse for StateFilterInputs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let input_name = input.parse::<Ident>()?;
        input.parse::<Comma>()?;

        let mut all_types = vec![input.parse::<Type>()?];
        while input.parse::<Comma>().is_ok() {
            all_types.push(input.parse::<Type>()?);
        }

        Ok(StateFilterInputs {
            input_name,
            all_types,
        })
    }
}

#[proc_macro]
pub fn impl_state_filter_inputs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StateFilterInputs);
    let input_name = input.input_name;
    let mut all_types = input.all_types;
    let mut input = Vec::with_capacity(all_types.len());
    let mut input_index = Vec::with_capacity(all_types.len());
    let mut tuples = Vec::with_capacity(all_types.len());
    let mut tuple_indexes = Vec::with_capacity(all_types.len());
    let mut remainder_tuples = Vec::with_capacity(all_types.len());
    for rotate_i in 0..all_types.len() {
        all_types.rotate_right(rotate_i);
        for (i, ty_0) in all_types.iter().enumerate() {
            input.push(ty_0.clone());
            let i_name = Index::from(i);
            input_index.push(quote::quote!(self.#i_name));
            let mut tys = Vec::with_capacity(all_types.len());
            let mut tuple_i = Vec::with_capacity(all_types.len() - 1);
            let mut remainders = Vec::with_capacity(all_types.len() - 1);
            for (i_1, ty_1) in all_types.iter().enumerate() {
                tys.push(ty_1);
                if i_1 != i {
                    tuple_i.push(Index::from(i_1));
                    remainders.push(ty_1);
                }
            }
            remainder_tuples.push(quote::quote!((#(#remainders),*)));
            tuple_indexes.push(quote::quote!((#(self.#tuple_i),*)));
            tuples.push(quote::quote!((#(#tys),*)));
        }
    }
    quote::quote! {
        pub struct #input_name<T: card_game::validation::StateFilterInput>(T);
        impl<T: card_game::validation::StateFilterInput> card_game::validation::StateFilterInput for #input_name<T> {}

        #(
            impl card_game::validation::StateFilterInputConversion<#input> for #input_name<#tuples>
            {
                type Remainder = #remainder_tuples;
                fn split_take(self) -> (#input, Self::Remainder) {
                    (#input_index, #tuple_indexes)
                }
            }
        )*
    }.into()
}

struct StateFilterCombination {
    name: Ident,
    start: LitInt,
    end: LitInt,
    generic: Ident,
}

impl Parse for StateFilterCombination {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name = input.parse::<Ident>()?;
        input.parse::<Comma>()?;

        let start = input.parse::<LitInt>()?;
        input.parse::<Comma>()?;

        let end = input.parse::<LitInt>()?;
        input.parse::<Comma>()?;

        let generic = input.parse::<Ident>()?;

        Ok(StateFilterCombination {
            name,
            start,
            end,
            generic,
        })
    }
}

#[proc_macro]
pub fn impl_state_filter_combination(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StateFilterCombination);
    let name = input.name;
    let macro_name = format_ident!("impl_state_filter_combination_for_{name}");
    let start = input.start;
    let end = input.end;
    let generic = input.generic;
    quote::quote! {
        macro_rules! #macro_name {
            ($(($i: tt, $t: ident)),*) => {
                impl<$($t,)*> card_game::validation::StateFilterCombination<($($t,)*)> for #name {
                    type Combined = ($($t,)* Self);
                    fn combine(self, value: ($($t,)*)) -> Self::Combined {
                        ($(value.$i,)* self)
                    }
                }
            };
        }
        card_game::variadics_please::all_tuples_enumerated!(#macro_name, #start, #end, #generic);
    }
    .into()
}
