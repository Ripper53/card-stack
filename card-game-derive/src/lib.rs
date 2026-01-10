extern crate proc_macro;

use heck::ToSnakeCase;
use proc_macro::TokenStream;
use syn::{
    DeriveInput, Field, Fields, Ident, Type, TypePath,
    parse::{Parse, Parser},
    parse_macro_input,
    visit_mut::VisitMut,
};

struct EventManagerArgs {
    states: StateMapping,
    events: Vec<EventMapping>,
}

struct StateMapping {
    states: Vec<syn::Type>,
    placeholder: syn::Ident,
}

struct EventMapping {
    event: syn::Ident,
    resolution: syn::Type,
}

impl Parse for EventManagerArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut states = None;
        let mut events = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            let _ = input.parse::<syn::Token![=]>()?;

            match key.to_string().as_str() {
                "states" => {
                    states = Some(input.parse()?);
                }
                "events" => {
                    let content;
                    syn::parenthesized!(content in input);
                    let parsed = content
                        .parse_terminated(EventMapping::parse, syn::Token![,])?
                        .into_iter()
                        .collect();
                    events = Some(parsed);
                }
                _ => {
                    return Err(syn::Error::new_spanned(key, "Unknown key"));
                }
            }

            let _ = input.parse::<syn::Token![,]>();
        }

        Ok(EventManagerArgs {
            states: states.ok_or_else(|| input.error("missing `states`"))?,
            events: events.ok_or_else(|| input.error("missing `events`"))?,
        })
    }
}

impl Parse for StateMapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        syn::parenthesized!(content in input);

        let states = content
            .parse_terminated(syn::Type::parse, syn::Token![,])?
            .into_iter()
            .collect();
        let _ = input.parse::<syn::Token![=>]>()?;
        let placeholder = input.parse::<syn::Ident>()?;

        Ok(StateMapping {
            states,
            placeholder,
        })
    }
}

impl Parse for EventMapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let event = input.parse::<syn::Ident>()?;
        let _ = input.parse::<syn::Token![=>]>()?;
        let resolution = input.parse::<syn::Type>()?;
        Ok(EventMapping { event, resolution })
    }
}

#[proc_macro_attribute]
pub fn event_manager(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EventManagerArgs);
    let mut ast = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &ast.ident;
    let mut impls = Vec::new();
    if let Fields::Named(ref mut fields) = ast.fields {
        for state in args.states.states {
            for event in args.events.iter() {
                {
                    let state_str = quote::quote!(#state).to_string().to_snake_case();
                    let state = syn::parse_quote!(card_game::stack::priority::Priority<#state>);
                    let event_resolution =
                        substitute_type(&event.resolution, &args.states.placeholder, &state);
                    let event = &event.event;
                    let field_name = quote::format_ident!(
                        "{}_during_{}",
                        event.to_string().to_snake_case(),
                        state_str,
                    );
                    let return_ty: syn::Type = syn::parse_quote!(
                        card_game::events::EventManager<
                            #state,
                            #event,
                            #event_resolution
                        >
                    );
                    let new_field = Field {
                        attrs: Vec::new(),
                        mutability: syn::FieldMutability::None,
                        vis: syn::Visibility::Inherited,
                        ident: Some(field_name.clone()),
                        colon_token: Some(<syn::Token![:]>::default()),
                        ty: return_ty.clone(),
                    };
                    fields.named.push(new_field);
                    impls.push(quote::quote! {
                        impl #struct_name {
                            pub fn #field_name(&self) -> &#return_ty {
                                &self.#field_name
                            }
                        }
                        impl card_game::events::AddEventListener<#state, #event> for #struct_name {
                            type Output = #event_resolution;
                            fn add_listener<
                                Listener: card_game::events::EventListener<#state, #event>,
                            >(
                                &mut self,
                                listener: Listener,
                            ) where
                                <Listener::Action as card_game::events::EventValidAction<
                                    card_game::stack::priority::PriorityMut<#state>,
                                    Listener::ActionInput,
                                >>::Output: Into<Self::Output>,
                            {
                                self.#field_name.add_listener(listener)
                            }
                        }
                    });
                }
                {
                    let state_str = quote::quote!(#state).to_string().to_snake_case();
                    let state = syn::parse_quote!(card_game::stack::priority::Priority<#state>);
                    let ev = &event.event;
                    let event_resolution =
                        substitute_type(&event.resolution, &args.states.placeholder, &state);
                    let state = syn::parse_quote!(card_game::events::EventPriorityStack<#state, #ev, #event_resolution>);
                    let event_resolution =
                        substitute_type(&event.resolution, &args.states.placeholder, &state);
                    let event = &event.event;
                    let field_name = quote::format_ident!(
                        "{}_stack_during_{}",
                        event.to_string().to_snake_case(),
                        state_str,
                    );
                    let return_ty: syn::Type = syn::parse_quote!(
                        card_game::events::EventManager<
                            #state,
                            #event,
                            #event_resolution
                        >
                    );
                    let new_field = Field {
                        attrs: Vec::new(),
                        mutability: syn::FieldMutability::None,
                        vis: syn::Visibility::Inherited,
                        ident: Some(field_name.clone()),
                        colon_token: Some(<syn::Token![:]>::default()),
                        ty: return_ty.clone(),
                    };
                    fields.named.push(new_field);
                    impls.push(quote::quote! {
                        impl #struct_name {
                            pub fn #field_name(&self) -> &#return_ty {
                                &self.#field_name
                            }
                        }
                        impl card_game::events::AddEventListener<#state, #event> for #struct_name {
                            type Output = #event_resolution;
                            fn add_listener<
                                Listener: card_game::events::EventListener<#state, #event>,
                            >(
                                &mut self,
                                listener: Listener,
                            ) where
                                <Listener::Action as card_game::events::EventValidAction<
                                    card_game::stack::priority::PriorityMut<#state>,
                                    Listener::ActionInput,
                                >>::Output: Into<Self::Output>,
                            {
                                self.#field_name.add_listener(listener)
                            }
                        }
                    });
                }
            }
        }
        quote::quote! {
            #ast
            #(#impls)*
        }
        .into()
    } else {
        panic!("`event_mananger` can only be used with named structs");
    }
}

fn substitute_type(ty: &Type, from: &Ident, to: &Type) -> Type {
    let mut out = ty.clone();
    TypeSubstituter { from, to }.visit_type_mut(&mut out);
    out
}

struct TypeSubstituter<'a> {
    from: &'a Ident,
    to: &'a Type,
}

impl<'a> syn::visit_mut::VisitMut for TypeSubstituter<'a> {
    fn visit_type_mut(&mut self, node: &mut Type) {
        match node {
            Type::Path(TypePath { path, .. })
                if path.segments.len() == 1 && &path.segments[0].ident == self.from =>
            {
                *node = self.to.clone();
            }
            _ => {
                syn::visit_mut::visit_type_mut(self, node);
            }
        }
    }
}
