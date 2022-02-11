use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Result, ItemImpl};

pub fn _main(input: ItemImpl) -> Result<TokenStream> {

    let res = quote! {
        #input
    };

    Ok(res.into())
}

pub fn rpc(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemImpl);

    match _main(parsed) {
        Ok(tk) => tk,
        Err(e) => e.to_compile_error().into(),
    }
}
