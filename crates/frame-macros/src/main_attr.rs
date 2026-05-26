use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub fn frame_main_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let block = &input.block;

    let expanded = quote! {
        #[cfg(not(target_os = "android"))]
        fn main() #block

        #[cfg(target_os = "android")]
        #[no_mangle]
        fn android_main(app: ::frame::AndroidApp) {
            ::frame::android::set_app(app);
            #block
        }
    };

    expanded.into()
}
