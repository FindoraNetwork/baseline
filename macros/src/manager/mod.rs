use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, Result, parse_macro_input};

mod module;
mod manager;
mod rpc;
mod clone;
mod block;
mod mempool;
mod genesis;

pub fn _manager(st: ItemStruct) -> Result<TokenStream> {
    let impl_manager = manager::impl_manager(&st)?;
    let impl_rpc = rpc::impl_rpc(&st)?;
    let impl_module = module::impl_module(&st)?;
    let impl_clone = clone::impl_clone(&st)?;
    let impl_block = block::impl_block(&st)?;
    let impl_mempool = mempool::impl_mempool(&st)?;
    let impl_genesis = genesis::impl_genesis(&st)?;

    let expand = quote! {
        #st

        #impl_clone

        #impl_module

        #impl_rpc

        #impl_block

        #impl_mempool

        #impl_genesis

        #impl_manager
    };

    Ok(expand.into())
}

pub fn manager(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);

    match _manager(parsed) {
        Ok(tk) => tk,
        Err(e) => e.to_compile_error().into(),
    }
}
