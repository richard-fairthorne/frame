mod ast;
mod codegen;
mod main_attr;
mod parser;
mod rsx;

use proc_macro::TokenStream;

#[proc_macro]
pub fn view(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    let element = match crate::parser::parse_element(input2) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = crate::codegen::gen_element(&element);

    expanded.into()
}

#[proc_macro]
pub fn rsx(input: TokenStream) -> TokenStream {
    let input2: proc_macro2::TokenStream = input.into();

    let element = match crate::rsx::parse_rsx(input2) {
        Ok(e) => e,
        Err(e) => return e.to_compile_error().into(),
    };

    let expanded = crate::codegen::gen_element(&element);
    expanded.into()
}

#[proc_macro_attribute]
pub fn frame_main(attr: TokenStream, item: TokenStream) -> TokenStream {
    crate::main_attr::frame_main_impl(attr, item)
}
