use proc_macro::TokenStream;

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
