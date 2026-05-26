use proc_macro2::TokenStream;

#[derive(Clone)]
pub struct Element {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub children: Vec<Child>,
}

#[derive(Clone)]
pub struct Attribute {
    pub name: String,
    pub value: TokenStream,
}

#[derive(Clone)]
pub enum Child {
    Element(Element),
    Text(String),
    Expr(TokenStream),
}
