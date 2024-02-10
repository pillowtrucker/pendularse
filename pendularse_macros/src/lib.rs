use proc_macro::TokenStream;

use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NannouModel)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let out = match input.data {
        syn::Data::Struct(_) => {
            panic!("enums only")
        }
        syn::Data::Enum(enum_data) => {
            let variants = &enum_data.variants;
            let imp_method = |method_name, method_args: &'static str| {
                variants.iter().map(move |v| {
                    let name = &v.ident;
                    let method_name = Ident::new(method_name, Span::call_site());
                    let method_args = if method_args.is_empty() {
                        vec![]
                    } else {
                        method_args
                            .split(',')
                            .map(|a| Ident::new(a, Span::call_site()))
                            .collect()
                    };
                    quote! {
                        #ident :: #name(inner) => inner.#method_name(#(#method_args),*)
                    }
                })
            };
            let imp_m_model = imp_method("model", "");
            let imp_m_update = imp_method("update", "");
            let imp_static_fn = |fn_name, fn_args: &'static str| {
                variants.iter().map(move |v| {
                    let name = &v.ident;
                    let fn_name = Ident::new(fn_name, Span::call_site());
                    let fn_args = if fn_args.is_empty() {
                        vec![]
                    } else {
                        fn_args
                            .split(',')
                            .map(|a| Ident::new(a, Span::call_site()))
                            .collect()
                    };
                    quote! {
                        #ident :: #name(inner) => #name :: #fn_name(#(#fn_args),*)
                    }
                })
            };
            let imp_view = imp_static_fn("view", "model");
            let imp_window_event = imp_static_fn("window_event", "model");

            quote! {
            impl NannouModel<#ident> for #ident {
                fn model(&self) -> nannou::app::ModelFn<#ident> {
                    match self {
                        #(#imp_m_model),*
                    }
                }
                fn update(&self) -> nannou::app::UpdateFn<#ident> {
                    match self {
                        #(#imp_m_update),*
                    }
                }
                fn view(model: &#ident) -> nannou::window::ViewFn<#ident> {
                    match model {
                        #(#imp_view),*
                    }
                }
                fn window_event(model: &#ident) -> nannou::window::EventFn<#ident> {
                    match model {
                        #(#imp_window_event),*
                    }
                }

            }}
        }
        syn::Data::Union(_) => {
            panic!("but we're just like a family in here. There's a fussball table and 5 kinds of sweetened cereal");
        }
    };
    out.into()
}
