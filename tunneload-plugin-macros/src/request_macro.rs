use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

use crate::util::{find_config_type, load_config};

fn call_request(handler_name: Ident, request_name: &Ident, result_name: &Ident) -> TokenStream {
    quote! {
        let #result_name: Result<(), tunneload_plugin::Response<'_>> = #handler_name(#request_name);
    }
}

fn call_config_request(
    handler_name: Ident,
    attributes: &[syn::NestedMeta],
    request_name: &Ident,
    result_name: &Ident,
) -> TokenStream {
    let config_type = match find_config_type(attributes) {
        Some(c) => c,
        None => {
            return quote! {
                compile_error!("Expected Config Type to specified in the Attribute-Macro");
            }
        }
    };

    let config_name = Ident::new("config", Span::call_site());
    let load_config_tokens = load_config(&config_name, config_type);

    quote! {
        #load_config_tokens

        let #result_name: Result<(), tunneload_plugin::Response<'_>> = #handler_name(#config_name, #request_name);
    }
}

fn return_response(resp_name: &Ident) -> TokenStream {
    quote! {
        // Serialize the Response to be able to send it to Tunneload
        let (head, body) = #resp_name.serialize();
        let total_length: u32 = (head.len() as u32) + (body.len() as u32);
        let total_length_bytes = total_length.to_be_bytes();

        // Reserve the Buffer needed to hold the entire Response
        let mut result_buffer = Vec::with_capacity(4 + total_length as usize);

        // First Copy the Length into the final Buffer
        result_buffer.extend_from_slice(&total_length_bytes);
        // Copy the request into the final Buffer
        result_buffer.extend_from_slice(&head);
        result_buffer.extend_from_slice(&body);

        let result_ptr = result_buffer.as_ptr() as i32;
        std::mem::forget(result_buffer);

        result_ptr
    }
}

pub fn parse_request(
    attributes: Vec<syn::NestedMeta>,
    input_fn: syn::ItemFn,
) -> proc_macro2::TokenStream {
    let handler_signature = input_fn.sig.clone();
    let handler_name = handler_signature.ident;

    let request_name = Ident::new("request", Span::call_site());
    let result_name = Ident::new("result", Span::call_site());

    let inputs = handler_signature.inputs;
    let call_tokens = match inputs.len() {
        1 => call_request(handler_name, &request_name, &result_name),
        2 => call_config_request(handler_name, &attributes, &request_name, &result_name),
        _ => {
            let span = inputs.span();
            return quote_spanned! {
                span => compile_error!("The handler Function should accept exactly 1 Input");
            };
        }
    };

    let response_name = Ident::new("resp", Span::call_site());

    let return_response_tokens = return_response(&response_name);

    quote! {
        #[no_mangle]
        pub extern "C" fn apply_req(config_size: i32, path_length: i32, body_size: i32, max_header_length: i32) -> i32 {
            #input_fn

            let #request_name = tunneload_plugin::MiddlewareRequest::new(path_length, body_size, max_header_length);

            #call_tokens

            match #result_name {
                Ok(_) => -1,
                Err(#response_name) => {
                    #return_response_tokens
                },
            }
        }
    }
}
