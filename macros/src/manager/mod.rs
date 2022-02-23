use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, ItemStruct, MetaNameValue, Result, Token};

mod attr;

pub fn _manager(st: ItemStruct, _arg: attr::Attr) -> Result<TokenStream> {
    //     let impl_manager = manager::impl_manager(&st)?;
    // let impl_rpc = rpc::impl_rpc(&st)?;
    // let impl_module = module::impl_module(&st, &arg)?;
    // let impl_clone = clone::impl_clone(&st)?;
    // let impl_block = block::impl_block(&st)?;
    // // let impl_mempool = mempool::impl_mempool(&st)?;
    // let impl_genesis = genesis::impl_genesis(&st)?;
    //
    let expand = quote! {
            #st

            // #impl_clone

            // #impl_module
            //
            // #impl_rpc
            //
            // #impl_block
            //
            // // #impl_mempool
            //
            // #impl_genesis
            //
    //         #impl_manager
        };

    Ok(expand.into())
}

pub fn manager(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);

    let buffer =
        parse_macro_input!(args with Punctuated<MetaNameValue, Token![,]>::parse_terminated);

    let args = match attr::Attr::from_meta(buffer) {
        Ok(args) => args,
        Err(e) => return e.to_compile_error().into(),
    };

    match _manager(parsed, args) {
        Ok(tk) => tk,
        Err(e) => e.to_compile_error().into(),
    }
}
