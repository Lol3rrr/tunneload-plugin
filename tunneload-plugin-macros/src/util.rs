use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{Meta, NestedMeta};

/// Attempts to find the Type of the Configuration for the Handler
pub fn find_config_type(attributes: &[syn::NestedMeta]) -> Option<Ident> {
    for tmp in attributes {
        let meta = match tmp {
            NestedMeta::Meta(meta) => meta,
            _ => continue,
        };

        let path = match meta {
            Meta::Path(path) => path,
            _ => continue,
        };

        let segments = &path.segments;
        if segments.is_empty() {
            continue;
        }
        return Some(segments[0].ident.clone());
    }

    None
}

/// The Tokens to load the Configuration from Tunneload.
///
/// # Params:
/// * `config_var_name`: The Name of the variable into which the Configuration will be loaded
/// * `config_type`: The Type of the Configuration
pub fn load_config(config_var_name: &Ident, config_type: Ident) -> TokenStream {
    quote! {
        let config_usize = config_size as usize;
        let mut buffer = Vec::with_capacity(config_usize);
        unsafe {
            tunneload_plugin::raw::get_config(buffer.as_ptr() as i32);
            buffer.set_len(config_usize);
        }

        let #config_var_name = #config_type::deserialize_data(buffer.as_mut_ptr(), config_usize).unwrap();
        std::mem::forget(buffer);
    }
}
