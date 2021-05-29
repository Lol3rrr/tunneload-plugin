use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident};

use crate::config_derive::util;

fn len_names(names: &[Ident]) -> TokenStream {
    let mut result = quote! {};
    for name in names.iter() {
        result.extend(quote! {
            + tunneload_plugin::Config::len(#name) + 4
        });
    }

    result
}

pub fn parse(input: &DeriveInput) -> TokenStream {
    let length_calc = match &input.data {
        Data::Struct(derive_struct) => {
            let mut length_calc = quote! { 0 };
            let names = util::names_from_field(&derive_struct.fields);
            for name in names.iter() {
                length_calc.extend(quote! {
                    + tunneload_plugin::Config::len(&self.#name) + 4
                });
            }

            length_calc
        }
        Data::Enum(derive_enum) => {
            let mut match_inner = quote! {};
            for tmp_var in &derive_enum.variants {
                let names = util::names_from_field(&tmp_var.fields);
                let var_match_name = util::self_enum_variant(&tmp_var.ident, &names);

                let len_calc = len_names(&names);

                match_inner.extend(quote! {
                    #var_match_name => 1 #len_calc,
                });
            }

            quote! {
                match &self {
                    #match_inner
                    _ => 0,
                }
            }
        }
        Data::Union(_derive_union) => {
            quote! {
                compile_error!("Unions are not yet supported");
            }
        }
    };

    quote! {
        fn len(&self) -> usize {
            #length_calc
        }
    }
}
