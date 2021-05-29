#![warn(missing_docs)]
//! This crate provides all the Macros for the Tunneload-Plugin system

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

mod config_derive;
mod middleware_plugin;
mod parse_macro;

/// Marks the function as the initial Parser for the Configuration
#[proc_macro_attribute]
pub fn parse_config(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let attributes = parse_macro_input!(attr as AttributeArgs);

    parse_macro::parse_parse(attributes, input_fn).into()
}

/// Automatically implements the Config trait on the given Type
#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input);

    config_derive::parse_derive(derive_input).into()
}

/// Exports the Middleware-Trait-Implementation with the Middleware-Plugin
/// API for usage in Tunneload
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn MiddlewarePlugin(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_impl: syn::ItemImpl = parse_macro_input!(input);
    let attributes = parse_macro_input!(attr as AttributeArgs);

    middleware_plugin::impl_middleware_plugin(attributes, input_impl).into()
}
