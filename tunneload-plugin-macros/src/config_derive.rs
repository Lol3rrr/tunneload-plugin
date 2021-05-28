use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod deserialize;
mod len;
mod serialize;
mod util;

pub fn parse_derive(input: DeriveInput) -> TokenStream {
    let type_name = input.ident.clone();

    // TODO
    // Currently all the Data is just packed together but there is no garantuee that the
    // Data is currently aligned or will be aligned on future reads.
    // This obviously makes the generated code more complex and adds extra overhead
    // as long as we dont find a nice way to remove this problem
    let serialize_tokens = serialize::parse(&input);
    let deserialize_tokens = deserialize::parse(&input);
    let len_tokens = len::parse(&input);

    quote! {
        impl tunneload_plugin::Config for #type_name {
            #serialize_tokens

            #deserialize_tokens

            #len_tokens
        }
    }
}
