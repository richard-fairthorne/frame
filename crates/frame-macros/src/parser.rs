use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::{Expr, Ident, Result, Token, braced};

use crate::ast::{Attribute, Child, Element};

struct ViewRoot {
    element: Element,
}

impl Parse for ViewRoot {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);
        let element = parse_body_content(&name.to_string(), &content)?;
        Ok(ViewRoot { element })
    }
}

pub fn parse_element(input: TokenStream) -> Result<Element> {
    let root: ViewRoot = syn::parse2(input)?;
    Ok(root.element)
}

fn parse_body_content(name: &str, content: &ParseBuffer) -> Result<Element> {
    let mut attributes = Vec::new();
    let mut children = Vec::new();

    while !content.is_empty() {
        let ident: Ident = content.parse()?;

        if content.peek(syn::token::Brace) {
            let inner;
            braced!(inner in content);
            let child = parse_body_content(&ident.to_string(), &inner)?;
            children.push(Child::Element(child));
        } else {
            content.parse::<Token![:]>()?;
            let value: Expr = content.parse()?;
            attributes.push(Attribute {
                name: ident.to_string(),
                value: value.into_token_stream(),
            });
        }

        if content.peek(Token![,]) {
            content.parse::<Token![,]>()?;
        }
    }

    Ok(Element {
        name: name.to_string(),
        attributes,
        children,
    })
}
