use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Ident, ItemFn, ReturnType};

use crate::util::{find_config_type, load_config};

fn call_response(handler_name: &Ident) -> TokenStream {
    quote! {
        #handler_name(response);
    }
}

fn call_config_response(handler_name: &Ident, attributes: &[syn::NestedMeta]) -> TokenStream {
    let config_type = match find_config_type(attributes) {
        Some(c) => c,
        None => {
            return quote! {
                compile_error!("Expected a Config type to be set on the Macro");
            }
        }
    };

    let config_name = Ident::new("config", Span::call_site());
    let load_config_tokens = load_config(&config_name, config_type);

    quote! {
        #load_config_tokens

        #handler_name(#config_name, response);
    }
}

pub fn parse_response(
    attributes: Vec<syn::NestedMeta>,
    input_fn: ItemFn,
) -> proc_macro2::TokenStream {
    let handler_signature = input_fn.sig.clone();
    let handler_name = handler_signature.ident;

    if let ReturnType::Type(_, _) = handler_signature.output {
        return quote! {
            compile_error!("Response-Handler should not return anything");
        };
    }

    let inputs = handler_signature.inputs;
    let calls_tokens = match inputs.len() {
        1 => call_response(&handler_name),
        2 => call_config_response(&handler_name, &attributes),
        _ => {
            let span = inputs.span();
            return quote_spanned! {
                span => compile_error!("Unknown quantity of Arguments for the Response-Handler");
            };
        }
    };

    let gen = quote! {
        #[no_mangle]
        pub extern "C" fn apply_resp(config_size: i32, body_size: i32, max_header_length: i32) {
            #input_fn

            let response = tunneload_plugin::MiddlewareResponse::new(body_size, max_header_length);

            #calls_tokens
        }
    };

    gen.into()
}
