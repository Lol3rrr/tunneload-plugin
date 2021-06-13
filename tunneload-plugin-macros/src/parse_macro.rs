use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, ItemFn, ReturnType};

pub fn parse_parse(
    _attributes: Vec<syn::NestedMeta>,
    input_fn: ItemFn,
) -> proc_macro2::TokenStream {
    let handler_signature = input_fn.sig.clone();
    let handler_name = handler_signature.ident;

    let inputs = handler_signature.inputs;
    if inputs.len() != 1 {
        let span = inputs.span();
        return quote_spanned! {
            span => compile_error!("The Config-Parse Function should accept exactly 1 Input");
        };
    }

    if let ReturnType::Default = handler_signature.output {
        return quote! {
            compile_error!("The Config-Parser should return a Type that implements the `tunneload_plugin::Config` trait");
        };
    }

    quote! {
        #[no_mangle]
        pub extern "C" fn parse_config(config_string_size: i32) -> i32 {
            #input_fn

            let config_string = tunneload_plugin::load_config_str(config_string_size);

            let config = #handler_name(config_string);

            let config_data = tunneload_plugin::Config::serialize_data(&config);
            let config_size = tunneload_plugin::Config::len(&config);

            let mut result_buffer = Vec::with_capacity(8);
            result_buffer.extend_from_slice(&((config_data.as_ptr()) as i32).to_be_bytes());
            result_buffer.extend_from_slice(&(config_size as i32).to_be_bytes());

            let buffer_ptr = result_buffer.as_ptr();
            std::mem::forget(result_buffer);
            std::mem::forget(config_data);

            buffer_ptr as i32
        }
    }
}
