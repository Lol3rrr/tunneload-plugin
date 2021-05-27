use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput, Fields, Type};

pub fn parse(input: &DeriveInput) -> TokenStream {
    let inner_deserialize = match &input.data {
        Data::Struct(derive_struct) => match &derive_struct.fields {
            Fields::Named(named_fields) => {
                let mut result = quote! {
                    use std::convert::TryInto;

                    let mut index = 0;
                    let mut buffer = unsafe { Vec::from_raw_parts(addr, size, size) };
                };

                for tmp in &named_fields.named {
                    let tmp_ident = tmp.ident.clone().unwrap();
                    let tmp_type = tmp.ty.clone();
                    let type_ident = match &tmp_type {
                        Type::Path(path_type) => {
                            let path = &path_type.path;

                            let mut result = quote! {};
                            for segment in path.segments.iter() {
                                result.extend(segment.ident.to_token_stream());
                            }
                            result
                        }
                        _ => {
                            eprintln!("Unknown Type: {:?}", tmp_type.to_token_stream().to_string());
                            tmp_type.to_token_stream()
                        }
                    };

                    let addition = quote! {
                        let tmp_slice_size = &buffer[index..index+4];
                        let size = (i32::from_be_bytes(tmp_slice_size.try_into().unwrap())) as usize;
                        index += 4;

                        let tmp_slice = &mut buffer[index..index + size];
                        let #tmp_ident = #type_ident::deserialize_data(tmp_slice.as_mut_ptr(), size)?;
                        index += size;
                    };
                    result.extend(addition);
                }

                let mut struct_creation = quote! {};
                for tmp in &named_fields.named {
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
            _ => quote! {
                None
            },
        },
        _ => quote! {
            None
        },
    };

    quote! {
        fn deserialize_data(addr: *mut u8, size: usize) -> Option<Self> {
            #inner_deserialize
        }
    }
}
