use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Result, ItemImpl, ImplItem, Error, Meta, Path, parse_quote};
use core::mem;

fn check_path_rpc(path: &Path) -> Result<()> {
    if path.get_ident().ok_or(Error::new(Span::call_site(), "get ident error"))?.to_string() == "rpc" {
        Ok(())
    } else {
        Err(Error::new(Span::call_site(), "rpc attr must be rpc"))
    }
}

pub fn _main(mut input: ItemImpl) -> Result<TokenStream> {

    // let mut v = Vec::new();

    for method in &mut input.items {
        if let ImplItem::Method(m) = method {
            let attrs = mem::take(&mut m.attrs);

            for attr in attrs {
                let args = attr.parse_meta()?;

                match args {
                    Meta::Path(p) => {
                        check_path_rpc(&p)?;
                    },
                    Meta::List(l) => {
                        check_path_rpc(&l.path)?;
                    },
                    _ => return Err(Error::new(Span::call_site(), "format error."))
                }
            }
        }
    }

    let ident = *input.self_ty.clone();

    let mut m: ItemImpl = parse_quote! {
        #[async_trait::async_trait]
        impl baseline::prelude::RPC for #ident {
            fn call(&mut self, req: baseline::types::rpc::Request) -> baseline::types::rpc::Response {
                let mut resp = baseline::types::rpc::Response::not_found();

                // match req.method {}

                resp
            }
        }
    };

    m.generics = input.generics.clone();

    let res = quote! {
        #input

        #m
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
