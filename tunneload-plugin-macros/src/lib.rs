use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

pub(crate) mod util;

mod config_derive;
mod parse_macro;
mod request_macro;
mod response_macro;

#[proc_macro_attribute]
pub fn request(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let attributes = parse_macro_input!(attr as AttributeArgs);

    request_macro::parse_request(attributes, input_fn).into()
}

#[proc_macro_attribute]
pub fn response(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let attributes = parse_macro_input!(attr as AttributeArgs);

    response_macro::parse_response(attributes, input_fn).into()
}

#[proc_macro_attribute]
pub fn parse_config(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let attributes = parse_macro_input!(attr as AttributeArgs);

    parse_macro::parse_parse(attributes, input_fn).into()
}

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input);

    config_derive::parse_derive(derive_input).into()
}
