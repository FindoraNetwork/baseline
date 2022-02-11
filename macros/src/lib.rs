use proc_macro::TokenStream;

mod module;
mod utils;

#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    module::module(args, input)
}
