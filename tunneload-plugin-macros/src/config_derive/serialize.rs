use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn parse(input: &DeriveInput) -> TokenStream {
    let inner_serialize = match &input.data {
        Data::Struct(derive_struct) => match &derive_struct.fields {
            Fields::Named(named_fields) => {
                let mut parsing = quote! {};
                for tmp in &named_fields.named {
                    let tmp_ident = tmp.ident.clone().unwrap();

                    let addition = quote! {
                        let tmp_serialized = tunneload_plugin::Config::serialize_data(&self.#tmp_ident);
                        let tmp_serialized_length = tmp_serialized.len();

                        buffer.extend_from_slice(&(tmp_serialized_length as i32).to_be_bytes());
                        buffer.extend(tmp_serialized);
                    };
                    parsing.extend(addition);
                }

                parsing
            }
            _ => {
                quote! {}
            }
        },
        _ => {
            quote! {}
        }
    };

    quote! {
        fn serialize_data(&self) -> Vec<u8> {
            let mut buffer = Vec::with_capacity(self.len());

            #inner_serialize

            buffer
        }
    }
}
