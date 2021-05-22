use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn request(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let handler_signature = input_fn.sig.clone();
    let handler_name = handler_signature.ident;

    // TODO
    // Check if the given function also accepts a MiddlewareRequest

    let gen = quote! {
        #[no_mangle]
        pub extern "C" fn apply_req() -> i32 {
            #input_fn

            let request = tunneload_plugin::MiddlewareRequest::new();

            // TODO
            // Change this to return a Response as Err value
            let result: Result<(), ()> = #handler_name(request);

            match result {
                Ok(_) => -1,
                // TODO
                // This should return the Error Response to
                // the Host
                Err(_) => 0,
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn response(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let handler_signature = input_fn.sig.clone();
    let handler_name = handler_signature.ident;

    let gen = quote! {
        #[no_mangle]
        pub extern "C" fn apply_resp() {
            #input_fn
            #handler_name();
        }
    };

    gen.into()
}
