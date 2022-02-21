use proc_macro::TokenStream;

mod manager;
mod module;
mod rpc;

mod utils;

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::module(args, input)
}

#[proc_macro_attribute]
pub fn rpc(args: TokenStream, input: TokenStream) -> TokenStream {
    rpc::rpc(args, input)
}

#[proc_macro_attribute]
pub fn manager(args: TokenStream, input: TokenStream) -> TokenStream {
    manager::manager(args, input)
}
