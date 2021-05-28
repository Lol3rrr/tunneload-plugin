use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

use crate::config_derive::util;

fn serialize_access(access_name: TokenStream) -> TokenStream {
    quote! {
        let tmp_serialized = tunneload_plugin::Config::serialize_data(#access_name);
        let tmp_serialized_length = tmp_serialized.len();

        buffer.extend_from_slice(&(tmp_serialized_length as i32).to_be_bytes());
        buffer.extend(tmp_serialized);
    }
}

fn parse_fields(fields: &Fields) -> TokenStream {
    match fields {
        Fields::Named(named_fields) => {
            let mut parsing = quote! {};
            for tmp in &named_fields.named {
                let tmp_ident = tmp.ident.clone().unwrap();

                let access = quote! {
                    &self.#tmp_ident
                };
                parsing.extend(serialize_access(access));
            }

            parsing
        }
        _ => {
            quote! {}
        }
    }
}

fn parse_names(names: &[Ident]) -> TokenStream {
    let mut result = quote! {};
    for name in names.iter() {
        let access = quote! {
            #name
        };
        result.extend(serialize_access(access));
    }

    result
}

pub fn parse(input: &DeriveInput) -> TokenStream {
    let inner_serialize = match &input.data {
        Data::Struct(derive_struct) => parse_fields(&derive_struct.fields),
        Data::Enum(derive_enum) => {
            let mut matching = quote! {};
            for (index, tmp) in derive_enum.variants.iter().enumerate() {
                let names = util::names_from_field(&tmp.fields);
                let variant_name = util::self_enum_variant(&tmp.ident, &names);
                let parsing = parse_names(&names);

                let serialize_index = index as u8;

                let addition = quote! {
                    #variant_name => {
                        buffer.push(#serialize_index);
                        #parsing
                    }
                };
                matching.extend(addition);
            }

            quote! {
                match self {
                    #matching
                };
            }
        }
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
