use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Fields, Ident};

pub fn names_from_field(fields: &Fields) -> Vec<Ident> {
    let mut result = Vec::new();

    match fields {
        Fields::Named(named_fields) => {
            for field in named_fields.named.iter() {
                let ident = field.ident.clone().unwrap();
                result.push(ident);
            }
        }
        Fields::Unnamed(unnamed_fields) => {
            for index in 0..unnamed_fields.unnamed.len() {
                let ident = Ident::new(&format!("tmp_{}", index), Span::call_site());
                result.push(ident);
            }
        }
        Fields::Unit => {}
    };

    result
}

pub fn self_enum_variant(variant_ident: &Ident, fields: &[Ident]) -> TokenStream {
    if fields.is_empty() {
        quote! {
            Self::#variant_ident
        }
    } else {
        let mut inner_names = quote! {};
        for (index, name) in fields.iter().enumerate() {
            if index == 0 {
                inner_names.extend(quote! { #name });
                continue;
            }

            inner_names.extend(quote! { , #name });
        }

        quote! {
            Self::#variant_ident(#inner_names)
        }
    }
}
