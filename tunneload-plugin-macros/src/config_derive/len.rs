use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

pub fn parse(input: &DeriveInput) -> TokenStream {
    let length_calc = match &input.data {
        Data::Struct(derive_struct) => match &derive_struct.fields {
            Fields::Named(named_fields) => {
                let mut length_calc = quote! { 0 };
                for tmp in &named_fields.named {
                    let tmp_ident = tmp.ident.clone().unwrap();

                    let addition = quote! {
                        + tunneload_plugin::Config::len(&self.#tmp_ident) + 4
                    };
                    length_calc.extend(addition);
                }

                length_calc
            }
            _ => {
                quote! {
                    0
                }
            }
        },
        _ => {
            quote! {
                0
            }
        }
    };

    quote! {
        fn len(&self) -> usize {
            #length_calc
        }
    }
}
