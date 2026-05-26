use proc_macro2::TokenStream;
use quote::ToTokens;
use rstml::node::{Node, NodeAttribute, NodeBlock, NodeName};

use crate::ast::{Attribute, Child, Element};

pub fn parse_rsx(input: TokenStream) -> Result<Element, syn::Error> {
    let nodes = rstml::parse2(input)?;

    if nodes.len() == 1 {
        Ok(node_to_element(&nodes[0]))
    } else {
        let children: Vec<Child> = nodes.iter().map(node_to_child).collect();
        Ok(Element {
            name: "Column".to_string(),
            attributes: vec![],
            children,
        })
    }
}

fn node_to_child(node: &Node) -> Child {
    match node {
        Node::Element(_) => Child::Element(node_to_element(node)),
        Node::Text(text) => Child::Text(text.value_string()),
        Node::Block(block) => match block {
            NodeBlock::ValidBlock(block) => Child::Expr(block.to_token_stream()),
            NodeBlock::Invalid(_) => Child::Text(String::new()),
        },
        Node::RawText(rt) => Child::Text(rt.to_string_best()),
        _ => Child::Text(String::new()),
    }
}

fn node_to_element(node: &Node) -> Element {
    let elem = match node {
        Node::Element(e) => e,
        _ => {
            return Element {
                name: "Text".to_string(),
                attributes: vec![],
                children: vec![],
            }
        }
    };

    let name = match &elem.open_tag.name {
        NodeName::Path(path) => path.to_token_stream().to_string().replace(' ', ""),
        _ => "Unknown".to_string(),
    };

    let attrs: Vec<Attribute> = elem
        .open_tag
        .attributes
        .iter()
        .filter_map(|attr| match attr {
            NodeAttribute::Attribute(keyed) => {
                let attr_name = keyed.key.to_token_stream().to_string().replace(' ', "");
                let value = keyed
                    .value()
                    .map(|e| e.to_token_stream())
                    .unwrap_or_else(|| quote::quote!(true));
                Some(Attribute {
                    name: attr_name,
                    value,
                })
            }
            NodeAttribute::Block(_) => None,
        })
        .collect();

    let raw_children: Vec<Child> = elem.children.iter().map(node_to_child).collect();

    let mut text_attr_from_child = None;
    let children = if name == "Text" && !attrs.iter().any(|a| a.name == "text") {
        let mut remaining = Vec::new();
        for child in raw_children {
            if text_attr_from_child.is_none() {
                match &child {
                    Child::Text(s) => {
                        text_attr_from_child = Some(Attribute {
                            name: "text".to_string(),
                            value: quote::quote!(#s),
                        });
                    }
                    Child::Expr(ts) => {
                        text_attr_from_child = Some(Attribute {
                            name: "text".to_string(),
                            value: ts.clone(),
                        });
                    }
                    Child::Element(_) => remaining.push(child),
                }
            } else {
                remaining.push(child);
            }
        }
        remaining
    } else {
        raw_children
    };

    let mut attributes = attrs;
    if let Some(attr) = text_attr_from_child {
        attributes.insert(0, attr);
    }

    Element {
        name,
        attributes,
        children,
    }
}
