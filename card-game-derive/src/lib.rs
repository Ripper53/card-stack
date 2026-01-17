extern crate proc_macro;

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro::TokenStream;
use syn::{
    DeriveInput, Field, Fields, Ident, Type, TypePath,
    parse::{Parse, Parser},
    parse_macro_input,
    spanned::Spanned,
    visit_mut::VisitMut,
};

struct EventManagerArgs {
    states: StateMapping,
    events: Vec<EventMapping>,
}

struct StateMapping {
    states: Vec<(syn::Type, syn::ExprClosure)>,
    placeholder: syn::Ident,
}

struct EventMapping {
    event: syn::Type,
    stackable: syn::Type,
    resolution: syn::Type,
    stackable_enum_types: Vec<IdentAndType>,
    resolution_enum_types: Vec<IdentAndType>,
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

        let closures = content
            .parse_terminated(syn::Expr::parse, syn::Token![,])?
            .into_iter()
            .collect::<Vec<_>>();

        input.parse::<syn::Token![as]>()?;
        let placeholder = input.parse::<Ident>()?;

        let mut states = Vec::new();

        for expr in closures {
            let closure = match expr {
                syn::Expr::Closure(closure) => closure,
                _ => {
                    return Err(syn::Error::new_spanned(
                        expr,
                        "expected a closure like `|state: Type| ...`",
                    ));
                }
            };

            if closure.inputs.len() != 1 {
                return Err(syn::Error::new_spanned(
                    &closure.inputs,
                    "closure must have exactly one parameter",
                ));
            }

            // Extract the type: |x: ShopStep|
            let state_ty = match closure.inputs.first().unwrap() {
                syn::Pat::Type(syn::PatType { ty, .. }) => match ty.as_ref() {
                    Type::Reference(syn::TypeReference { elem, .. }) => (**elem).clone(),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            ty,
                            "expected a reference type like `&StateType`",
                        ));
                    }
                },
                _ => {
                    return Err(syn::Error::new_spanned(
                        &closure.inputs,
                        "closure parameter must be typed: `|state: Type|`",
                    ));
                }
            };

            states.push((state_ty, closure));
        }

        Ok(StateMapping {
            states,
            placeholder,
        })
    }
}

fn closure_to_item_fn(
    closure: syn::ExprClosure,
    fn_name: &str,
    event_manager_name: Type,
) -> syn::Result<syn::ItemFn> {
    // Reject capturing closures
    if closure.capture.is_some() {
        return Err(syn::Error::new(
            closure.span(),
            "cannot convert capturing closure into function",
        ));
    }

    // Convert closure inputs into fn inputs
    let inputs = closure
        .inputs
        .into_iter()
        .map(|pat| match pat {
            syn::Pat::Type(pat_ty) => Ok(syn::FnArg::Typed(pat_ty)),
            _ => Err(syn::Error::new_spanned(
                pat,
                "closure parameters must be typed",
            )),
        })
        .collect::<syn::Result<_>>()?;

    let sig = syn::Signature {
        constness: closure.constness,
        asyncness: closure.asyncness,
        unsafety: None,
        abi: None,
        fn_token: Default::default(),
        ident: Ident::new(fn_name, proc_macro2::Span::call_site()),
        generics: Default::default(),
        paren_token: Default::default(),
        inputs,
        variadic: None,
        output: syn::ReturnType::Type(
            syn::token::RArrow::default(),
            Box::new(syn::parse_quote!(&#event_manager_name)),
        ),
    };

    Ok(syn::ItemFn {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        sig,
        block: Box::new(match *closure.body {
            syn::Expr::Block(block) => block.block,
            expr => syn::Block {
                brace_token: syn::token::Brace::default(),
                stmts: vec![syn::Stmt::Expr(expr, None)],
            },
        }),
    })
}

impl Parse for EventMapping {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let event = input.parse::<syn::Type>()?;
        let _ = input.parse::<syn::Token![^]>()?;

        let stackable = input.parse::<syn::Type>()?;

        let stackable_enum_types = if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            content
                .parse_terminated(IdentAndType::parse, syn::Token![,])?
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        let _ = input.parse::<syn::Token![=>]>()?;
        let resolution = input.parse::<syn::Type>()?;

        let resolution_enum_types = if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            content
                .parse_terminated(IdentAndType::parse, syn::Token![,])?
                .into_iter()
                .collect::<Vec<_>>()
        } else {
            Vec::new()
        };

        Ok(EventMapping {
            event,
            stackable,
            resolution,
            stackable_enum_types,
            resolution_enum_types,
        })
    }
}

struct IdentAndType {
    ident: Option<Ident>,
    ty: Type,
}
impl Parse for IdentAndType {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek2(syn::Token![:]) {
            let ident = input.parse()?;
            let _ = input.parse::<syn::Token![:]>()?;
            let ty = input.parse()?;
            Ok(IdentAndType {
                ident: Some(ident),
                ty,
            })
        } else {
            Ok(IdentAndType {
                ident: None,
                ty: input.parse()?,
            })
        }
    }
}

#[proc_macro_attribute]
pub fn event_manager(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as EventManagerArgs);
    let mut ast = parse_macro_input!(input as syn::ItemStruct);
    let struct_name = &ast.ident;
    let mut impls = Vec::new();
    if let Fields::Named(ref mut fields) = ast.fields {
        for (state, get_event_manager) in args.states.states {
            let get_event_manager = closure_to_item_fn(
                get_event_manager,
                "get_event_manager",
                syn::parse_quote!(#struct_name),
            )
            .expect("failed to make function from closure");
            let original_state = &state;
            for event in args.events.iter() {
                {
                    let state_str = quote::quote!(#state).to_string();
                    let state = syn::parse_quote!(card_game::stack::priority::Priority<#state>);
                    let event_resolution =
                        substitute_type(&event.resolution, &args.states.placeholder, &state);
                    let stackable =
                        substitute_type(&event.stackable, &args.states.placeholder, &state);
                    let event = &event.event;
                    let event_name = quote::quote!(#event).to_string();
                    let field_name = quote::format_ident!(
                        "{}_during_{}",
                        event_name.to_snake_case(),
                        state_str.to_snake_case(),
                    );
                    let return_ty: syn::Type = syn::parse_quote!(
                        card_game::events::EventManager<
                            #state,
                            #event,
                            #event_resolution<#state>,
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
                        impl card_game::events::Event<#original_state> for #event {
                            type Stackable = #stackable<#original_state, card_game::events::EventAction<#state, Self, #event_resolution<#state>>>;
                        }
                        impl card_game::events::GetEventManager<#event> for #original_state {
                            type Output = #event_resolution<#state>;
                            fn event_manager(
                                &self,
                            ) -> card_game::events::EventManager<card_game::stack::priority::Priority<Self>, #event, Self::Output> {
                                #get_event_manager
                                let event_manager = get_event_manager(self);
                                event_manager
                                    .#field_name()
                                    .clone()
                            }
                        }
                        impl card_game::events::AddEventListener<#state, #event> for #struct_name {
                            type Output = #event_resolution<#state>;
                            fn add_listener<
                                Listener: card_game::events::EventListener<#state, #event>,
                            >(
                                &mut self,
                                listener: Listener,
                            ) -> card_game::cards::EventManagerIndex where
                                <Listener::Action as card_game::events::EventValidAction<
                                    card_game::stack::priority::PriorityMut<#state>,
                                    Listener::ActionInput,
                                >>::Output: Into<Self::Output>,
                            {
                                self.#field_name.add_listener(listener)
                            }
                        }
                        impl ::std::convert::From<#state> for #event_resolution<#state> {
                            fn from(value: #state) -> Self {
                                Self::State(value)
                            }
                        }
                    });
                }
                {
                    let state_str = quote::quote!(#state).to_string().to_snake_case();
                    let priority_state =
                        syn::parse_quote!(card_game::stack::priority::Priority<#state>);
                    let original_event = &event.event;
                    let original_event_name =
                        quote::quote!(#original_event).to_string().to_snake_case();
                    let original_priority_event_resolution = substitute_type(
                        &event.resolution,
                        &args.states.placeholder,
                        &priority_state,
                    );
                    let state = syn::parse_quote!(card_game::events::EventPriorityStack<#state, #original_event, #original_priority_event_resolution<#priority_state>>);
                    for ev in args.events.iter() {
                        let event_name = {
                            let event = &ev.event;
                            quote::quote!(#event).to_string().to_snake_case()
                        };
                        let event_resolution = &ev.resolution;
                        let field_name = if *event_resolution == event.resolution {
                            quote::format_ident!("{}_stack_during_{}", event_name, state_str)
                        } else {
                            quote::format_ident!(
                                "{}_stack_during_{}_{}",
                                event_name,
                                state_str,
                                original_event_name,
                            )
                        };
                        let event = &ev.event;
                        let event_resolution = substitute_type(
                            event_resolution,
                            &args.states.placeholder,
                            &priority_state,
                        );
                        let event_resolution = substitute_type(
                            &ev.resolution,
                            &args.states.placeholder,
                            &syn::parse_quote!(card_game::events::EventAction<
                                #priority_state,
                                #event,
                                #event_resolution,
                            >),
                        );
                        let stack_event_resolution =
                            substitute_type(&ev.resolution, &args.states.placeholder, &state);
                        let stackable =
                            substitute_type(&ev.stackable, &args.states.placeholder, &state);
                        let return_ty: syn::Type = syn::parse_quote!(
                            card_game::events::EventManager<
                                #state,
                                #event,
                                #stack_event_resolution<#state>,
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
                            impl card_game::events::Event<
                                card_game::events::EventPriorityStack<#original_state, #original_event, #original_priority_event_resolution<#priority_state>>
                            > for #event {
                                type Stackable = #stackable<#original_state, card_game::events::EventAction<#priority_state, #original_event, #original_priority_event_resolution<#priority_state>>>;
                            }
                            impl card_game::events::GetStackEventManager<
                                #event,
                                card_game::events::EventAction<card_game::stack::priority::Priority<#original_state>, #original_event, #original_priority_event_resolution<#priority_state>>,
                            > for #original_state {
                                type Output = #stack_event_resolution<#state>;
                                fn stack_event_manager(
                                    &self,
                                ) -> card_game::events::EventManager<#state, #event, Self::Output> {
                                    #get_event_manager
                                    let event_manager = get_event_manager(self);
                                    event_manager
                                        .#field_name()
                                        .clone()
                                }
                            }
                            impl card_game::events::AddEventListener<#state, #event> for #struct_name {
                                type Output = #stack_event_resolution<#state>;
                                fn add_listener<
                                    Listener: card_game::events::EventListener<#state, #event>,
                                >(
                                    &mut self,
                                    listener: Listener,
                                ) -> card_game::cards::EventManagerIndex where
                                    <Listener::Action as card_game::events::EventValidAction<
                                        card_game::stack::priority::PriorityMut<#state>,
                                        Listener::ActionInput,
                                    >>::Output: Into<Self::Output>,
                                {
                                    self.#field_name.add_listener(listener)
                                }
                            }
                            impl ::std::convert::From<#state> for #stack_event_resolution<#state> {
                                fn from(value: #state) -> Self {
                                    Self::State(value)
                                }
                            }
                        });
                    }
                }
            }
        }
        for event in args.events.iter() {
            let stackable = &event.stackable;
            let stackable_enum_types = event
                .stackable_enum_types
                .iter()
                .map(|ty| substitute_type(&ty.ty, &args.states.placeholder, &syn::parse_quote!(card_game::stack::priority::PriorityStack<State, IncitingAction>)))
                .collect::<Vec<_>>();
            let stackable_enum_variant_names = event
                .stackable_enum_types
                .iter()
                .map(|ty| {
                    if let Some(ref ident) = ty.ident {
                        ident
                    } else {
                        type_to_ident(&ty.ty)
                    }
                })
                .collect::<Vec<_>>();
            let stackable_enum_variant_names_str = stackable_enum_variant_names
                .iter()
                .map(|ident| ident.to_string())
                .collect::<Vec<_>>();
            let stackable_enum_variants = event.stackable_enum_types
                .iter()
                .map(|ty| {
                    let ident = if let Some(ref ident) = ty.ident {
                        ident
                    } else {
                        type_to_ident(&ty.ty)
                    };
                    let ty = substitute_type(
                        &ty.ty,
                        &args.states.placeholder,
                        &syn::parse_quote!(card_game::stack::priority::PriorityStack<State, IncitingAction>));
                    quote::quote!(#ident(#ty))
                })
                .collect::<Vec<_>>();
            let resolution = &event.resolution;
            let resolution_enum_types = event
                .resolution_enum_types
                .iter()
                .map(|ty| {
                    substitute_type(&ty.ty, &args.states.placeholder, &syn::parse_quote!(State))
                })
                .collect::<Vec<_>>();
            let resolution_enum_variant_names = event
                .resolution_enum_types
                .iter()
                .map(|ty| {
                    let ident = if let Some(ref ident) = ty.ident {
                        ident.to_string()
                    } else {
                        type_to_ident(&ty.ty).to_string()
                    };
                    quote::format_ident!("{}", ident.to_upper_camel_case())
                })
                .collect::<Vec<_>>();
            let resolution_enum_variants = event
                .resolution_enum_types
                .iter()
                .map(|ty| {
                    let ident = if let Some(ref ident) = ty.ident {
                        ident.to_string()
                    } else {
                        type_to_ident(&ty.ty).to_string()
                    };
                    let enum_ty = quote::format_ident!("{}", ident.to_upper_camel_case());
                    let ty = substitute_type(
                        &ty.ty,
                        &args.states.placeholder,
                        &syn::parse_quote!(State),
                    );
                    quote::quote!(#enum_ty(#ty))
                })
                .collect::<Vec<_>>();
            let (
                events,
                (
                    event_resolutions,
                    (
                        stackable_event_names,
                        (
                            stackable_event_names_str,
                            (
                                resolution_triggered_event_variant_names,
                                resolution_triggered_event_variant_types,
                            ),
                        ),
                    ),
                ),
            ): (Vec<_>, (Vec<_>, (Vec<_>, (Vec<_>, (Vec<_>, Vec<_>))))) = args
                .events
                .iter()
                .map(|args| {
                    let event = &args.event;
                    let event_name = quote::quote!(#event).to_string();
                    let stackable_event_name_str =
                        format!("{}Event", event_name.to_upper_camel_case());
                    let stackable_event_name = quote::format_ident!("{stackable_event_name_str}");
                    (
                        event,
                        (
                            &args.resolution,
                            (
                                stackable_event_name,
                                (stackable_event_name_str,
                                (
                                    quote::format_ident!(
                                        "Triggered{}Event",
                                        quote::quote!(#event).to_string().to_upper_camel_case()
                                    ),
                                    quote::quote!(card_game::events::TriggeredEvent<State, #event>),
                                )),
                            ),
                        ),
                    )
                })
                .unzip();
            let stackable_event_constraints = quote::quote! {
                #(
                    #events: card_game::events::Event<card_game::stack::priority::PriorityMut<
                        card_game::stack::priority::PriorityStack<State, IncitingAction>,
                    >>,
                )*
            };
            let resolution_event_constraints = quote::quote! {
                #(
                    #events: card_game::events::Event<card_game::stack::priority::PriorityMut<State>>,
                )*
            };
            impls.push(quote::quote! {
                pub enum #stackable<State, IncitingAction: card_game::stack::actions::IncitingActionInfo<State>>
                    where #stackable_event_constraints
                {
                    #(
                        #stackable_event_names(card_game::events::EventAction<
                            card_game::stack::priority::PriorityStack<State, IncitingAction>,
                            #events,
                            #event_resolutions<
                                card_game::stack::priority::PriorityStack<State, IncitingAction>,
                            >,
                        >),
                    )*
                    #(#stackable_enum_variants),*
                }
                impl<State, IncitingAction: card_game::stack::actions::IncitingActionInfo<State>> ::std::fmt::Debug for #stackable<State, IncitingAction>
                    where IncitingAction::Stackable: ::std::fmt::Debug, #stackable_event_constraints
                        #(
                            #stackable_enum_types: ::std::fmt::Debug,
                        )*
                {
                    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::result::Result<(), ::std::fmt::Error> {
                        match self {
                            #(
                                Self::#stackable_event_names(value) => f.debug_tuple(#stackable_event_names_str).field(value).finish(),
                            )*
                            #(
                                Self::#stackable_enum_variant_names(value) => f.debug_tuple(#stackable_enum_variant_names_str).field(value).finish(),
                            )*
                        }
                    }
                }
                impl<State, IncitingAction: card_game::stack::actions::IncitingActionInfo<State>> ::std::clone::Clone for #stackable<State, IncitingAction>
                    where IncitingAction::Stackable: ::std::clone::Clone, #stackable_event_constraints
                        #(
                            #stackable_enum_types: ::std::clone::Clone,
                        )*
                {
                    fn clone(&self) -> Self {
                        match self {
                            #(
                                Self::#stackable_event_names(value) => Self::#stackable_event_names(value.clone()),
                            )*
                            #(
                                Self::#stackable_enum_variant_names(value) => Self::#stackable_enum_variant_names(value.clone()),
                            )*
                        }
                    }
                }
                #(
                    impl<
                        State,
                        IncitingAction: card_game::stack::actions::IncitingActionInfo<State>,
                    > ::std::convert::From<card_game::events::EventAction<
                        card_game::stack::priority::PriorityStack<State, IncitingAction>,
                        #events,
                        #event_resolutions<
                            card_game::stack::priority::PriorityStack<State, IncitingAction>,
                        >,
                    >> for #stackable<State, IncitingAction>
                        where #stackable_event_constraints
                    {
                        fn from(value: card_game::events::EventAction<
                            card_game::stack::priority::PriorityStack<State, IncitingAction>,
                            #events,
                            #event_resolutions<
                                card_game::stack::priority::PriorityStack<State, IncitingAction>,
                            >,
                        >) -> Self {
                            Self::#stackable_event_names(value)
                        }
                    }
                )*
                #(
                    impl<State, IncitingAction: card_game::stack::actions::IncitingActionInfo<State>> ::std::convert::From<#stackable_enum_types> for #stackable<State, IncitingAction>
                        where #stackable_event_constraints
                    {
                        fn from(value: #stackable_enum_types) -> Self {
                            Self::#stackable_enum_variant_names(value)
                        }
                    }
                )*
                #[derive(Debug, Clone)]
                pub enum #resolution<State>
                    where #resolution_event_constraints
                {
                    State(State),
                    #(#resolution_triggered_event_variant_names(#resolution_triggered_event_variant_types),)*
                    #(#resolution_enum_variants),*
                }
                #(
                    impl<State> ::std::convert::From<#resolution_triggered_event_variant_types> for #resolution<State>
                        where #resolution_event_constraints
                    {
                        fn from(value: #resolution_triggered_event_variant_types) -> Self {
                            Self::#resolution_triggered_event_variant_names(value)
                        }
                    }
                )*
                #(
                    impl<State> ::std::convert::From<#resolution_enum_types> for #resolution<State>
                        where #resolution_event_constraints
                    {
                        fn from(value: #resolution_enum_types) -> Self {
                            Self::#resolution_enum_variant_names(value)
                        }
                    }
                )*
            });
        }
        quote::quote! {
            #ast
            #(#impls)*
        }
        .into()
    } else {
        panic!("`event_manager` can only be used with named structs");
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

fn type_to_ident(ty: &Type) -> &Ident {
    match ty {
        Type::Path(type_path) => type_path
            .path
            .segments
            .last()
            .map(|seg| &seg.ident)
            .unwrap(),
        _ => unimplemented!(),
    }
}
