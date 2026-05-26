use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::{Child, Element};

pub fn gen_element(element: &Element) -> TokenStream {
    let mut counter = 0usize;
    gen_element_inner(element, &mut counter)
}

fn gen_element_inner(element: &Element, counter: &mut usize) -> TokenStream {
    let my_id = *counter;
    *counter += 1;

    let var_name = syn::Ident::new(
        &format!("__w{}", my_id),
        proc_macro2::Span::call_site(),
    );

    let constructor = gen_constructor(element);

    let attr_calls: Vec<TokenStream> = element
        .attributes
        .iter()
        .filter(|attr| !is_constructor_arg(&element.name, &attr.name))
        .map(|attr| {
            let attr_ident = syn::Ident::new(&attr.name, proc_macro2::Span::call_site());
            let value = &attr.value;
            quote! { .#attr_ident(#value) }
        })
        .collect();

    let child_exprs: Vec<TokenStream> = element
        .children
        .iter()
        .map(|child| match child {
            Child::Element(elem) => {
                let child_tokens = gen_element_inner(elem, counter);
                quote! { let #var_name = #var_name.child(#child_tokens); }
            }
            Child::Text(s) => {
                quote! { let #var_name = #var_name.child(frame_ui::Text::new(#s)); }
            }
            Child::Expr(ts) => {
                let expr = ts;
                quote! { let #var_name = #var_name.child(#expr); }
            }
        })
        .collect();

    if child_exprs.is_empty() {
        quote! {
            {
                let #var_name = #constructor #(#attr_calls)*;
                #var_name
            }
        }
    } else {
        quote! {
            {
                let mut #var_name = #constructor #(#attr_calls)*;
                #(#child_exprs)*
                #var_name
            }
        }
    }
}

fn gen_constructor(element: &Element) -> TokenStream {
    let name_ident = syn::Ident::new(&element.name, proc_macro2::Span::call_site());

    match element.name.as_str() {
        "Text" => {
            let text_arg = element
                .attributes
                .iter()
                .find(|attr| attr.name == "text");
            if let Some(arg) = text_arg {
                let value = &arg.value;
                quote! { #name_ident::new(#value) }
            } else {
                quote! { #name_ident::new("") }
            }
        }
        "Button" => {
            let label_arg = element
                .attributes
                .iter()
                .find(|attr| attr.name == "label");
            let onclick_arg = element
                .attributes
                .iter()
                .find(|attr| attr.name == "on_click");
            match (label_arg, onclick_arg) {
                (Some(label), Some(onclick)) => {
                    let lv = &label.value;
                    let cv = &onclick.value;
                    quote! { #name_ident::new(#lv, #cv) }
                }
                (Some(label), None) => {
                    let lv = &label.value;
                    quote! { #name_ident::new(#lv, || {}) }
                }
                (None, Some(onclick)) => {
                    let cv = &onclick.value;
                    quote! { #name_ident::new("", #cv) }
                }
                (None, None) => {
                    quote! { #name_ident::new("", || {}) }
                }
            }
        }
        _ => {
            quote! { #name_ident::new() }
        }
    }
}

fn is_constructor_arg(widget: &str, attr: &str) -> bool {
    match widget {
        "Text" => attr == "text",
        "Button" => matches!(attr, "label" | "on_click"),
        _ => false,
    }
}
