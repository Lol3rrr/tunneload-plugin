use proc_macro2::TokenStream;
use quote::quote;
use syn::{AttributeArgs, ItemImpl};

fn extern_run(impl_block: &ItemImpl) -> TokenStream {
    let type_name = &impl_block.self_ty;

    quote! {
        #[no_mangle]
        pub extern "C" fn accept() -> i32 {
            <#type_name as tunneload_plugin::Acceptor>::run();

            0
        }
    }
}

pub fn impl_acceptor_plugin(_attributes: AttributeArgs, impl_block: ItemImpl) -> TokenStream {
    let extern_block = extern_run(&impl_block);

    quote! {
        #impl_block

        #extern_block
    }
}
