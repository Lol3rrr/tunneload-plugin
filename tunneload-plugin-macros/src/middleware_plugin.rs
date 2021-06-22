use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{AttributeArgs, Ident, ImplItem, ItemImpl, Type};

fn return_response() -> TokenStream {
    quote! {
        // Serialize the Response to be able to send it to Tunneload
        let (head, body) = resp.serialize();
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

fn load_config(config_type: &Type, config_name: &Ident) -> TokenStream {
    quote! {
        let config_usize = if config_size > 0 { config_size as usize } else { 1 };
        let mut buffer = Vec::with_capacity(config_usize);
        unsafe {
            tunneload_plugin::raw::get_config(buffer.as_ptr() as i32);
            buffer.set_len(config_usize);
        }

        let #config_name = <#config_type>::deserialize_data(&buffer).unwrap();
        std::mem::forget(buffer);
    }
}

fn request_impl(config_type: &Type, config_name: &Ident) -> TokenStream {
    let return_response_tokens = return_response();

    let config_tokens = load_config(config_type, config_name);

    quote! {
        #[no_mangle]
        pub extern "C" fn apply_req(config_size: i32, path_length: i32, body_size: i32, max_header_length: i32) -> i32 {
            #config_tokens

            let request = tunneload_plugin::MiddlewareRequest::new(path_length, body_size, max_header_length);

            let result: Result<(), tunneload_plugin::Response<'_>> = #config_name.handle_request(request);

            match result {
                Ok(_) => -1,
                Err(resp) => {
                    #return_response_tokens
                }
            }
        }
    }
}

fn response_impl(config_type: &Type, config_name: &Ident) -> TokenStream {
    let config_tokens = load_config(config_type, config_name);

    quote! {
        #[no_mangle]
        pub extern "C" fn apply_resp(config_size: i32, req_path_length: i32, req_body_size: i32, req_max_header_length: i32, resp_body_size: i32, resp_max_header_length: i32) {
            #config_tokens

            let request = tunneload_plugin::MiddlewareRequest::new(req_path_length, req_body_size, req_max_header_length);
            let response = tunneload_plugin::MiddlewareResponse::new(resp_body_size, resp_max_header_length);

            #config_name.handle_response(&request, response);
        }
    }
}

pub fn impl_middleware_plugin(_attributes: AttributeArgs, impl_block: ItemImpl) -> TokenStream {
    let own_type = &impl_block.self_ty;
    let config_name = Ident::new("config", Span::call_site());

    let mut extern_block = quote! {};
    for item in impl_block.items.iter() {
        if let ImplItem::Method(method) = item {
            let method_ident = &method.sig.ident;

            if method_ident == "handle_request" {
                extern_block.extend(request_impl(&own_type, &config_name));
            }
            if method_ident == "handle_response" {
                extern_block.extend(response_impl(&own_type, &config_name));
            }
        }
    }

    quote! {
        #impl_block

        #extern_block
    }
}
