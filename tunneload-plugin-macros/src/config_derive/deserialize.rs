use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DataEnum, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Type};

use crate::config_derive::util;

fn parse_code(result_ident: &Ident, ty: &Type) -> TokenStream {
    quote! {
        let tmp_slice_size = &buffer[index..index+4];
        let size = (i32::from_be_bytes(tmp_slice_size.try_into().unwrap())) as usize;
        index += 4;

        let tmp_slice = &buffer[index..index + size];
        let #result_ident = <#ty>::deserialize_data(tmp_slice)?;
        index += size;
    }
}

fn parse_named_field(field: &Field) -> TokenStream {
    let tmp_ident = field.ident.clone().unwrap();
    let tmp_type = field.ty.clone();

    parse_code(&tmp_ident, &tmp_type)
}

fn parse_named(fields: &FieldsNamed) -> TokenStream {
    let mut result = quote! {
        use std::convert::TryInto;

        let mut index = 0;
    };

    for tmp in fields.named.iter() {
        result.extend(parse_named_field(tmp));
    }

    let mut struct_creation = quote! {};
    for tmp in fields.named.iter() {
        let tmp_ident = tmp.ident.clone().unwrap();

        let addition = quote! {
            #tmp_ident,
        };
        struct_creation.extend(addition);
    }
    result.extend(quote! {
        Some(Self {
            #struct_creation
        })
    });

    result
}

fn parse_unnamed(fields: &FieldsUnnamed) -> TokenStream {
    for tmp in fields.unnamed.iter() {
        eprintln!("[Deserialize] Unnamed-Field: {:#?}", tmp.to_token_stream());
    }

    quote! {
        None
    }
}

fn parse_enum(derive_enum: &DataEnum) -> TokenStream {
    let mut inner_match = quote! {};
    for (index, tmp_var) in derive_enum.variants.iter().enumerate() {
        let names = util::names_from_field(&tmp_var.fields);
        let result_build = util::self_enum_variant(&tmp_var.ident, &names);

        let mut inner_parsing = quote! {};
        for (index, tmp_field) in tmp_var.fields.iter().enumerate() {
            let name = names.get(index).unwrap();
            let field_type = &tmp_field.ty;

            inner_parsing.extend(parse_code(name, field_type));
        }

        inner_parsing.extend(quote! {
            Some(#result_build)
        });

        let match_ident = index as u8;
        inner_match.extend(quote! {
            #match_ident => {
                #inner_parsing
            },
        })
    }

    quote! {
        use std::convert::TryInto;

        let first_byte = buffer[0];
        let mut index = 1;
        match first_byte {
            #inner_match
            _ => None,
        }
    }
}

pub fn parse(input: &DeriveInput) -> TokenStream {
    let inner_deserialize = match &input.data {
        Data::Struct(derive_struct) => match &derive_struct.fields {
            Fields::Named(named_fields) => parse_named(named_fields),
            Fields::Unnamed(unnamed_fields) => parse_unnamed(unnamed_fields),
            _ => quote! {
                None
            },
        },
        Data::Enum(derive_enum) => parse_enum(derive_enum),
        _ => quote! {
            None
        },
    };

    quote! {
        fn deserialize_data(buffer: &[u8]) -> Option<Self> {
            #inner_deserialize
        }
    }
}
