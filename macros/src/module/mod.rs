mod ctx;
mod metadata;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Fields, ItemStruct, Result, Error};

use core::panic;
use std::mem;

pub fn _module(mut parsed: ItemStruct) -> Result<TokenStream> {
    let mut outer_impls = Vec::new();

    let mut ctx_type = None;

    if let Fields::Named(n) = &mut parsed.fields {
        for field in &mut n.named {
            let attrs = mem::take(&mut field.attrs);

            for attr in attrs {
                let path = attr
                    .path
                    .get_ident()
                    .ok_or(Error::new(Span::call_site(), "read attr failed."))?
                    .to_string();

                match path.as_str() {
                    "context" => {
                        let ctx = ctx::impl_ctx(field.clone(), &parsed.ident, &parsed.generics);

                        ctx_type = Some(ctx.1);
                        outer_impls.push(ctx.0);
                    }
                    "metadata" => {
                        let item =
                            metadata::impl_metadata(attr, &parsed.ident, &parsed.generics)?;

                        outer_impls.push(item);
                    }
                    _ => panic!("no {} support, pleasr remove it", path),
                }
            }
        }
    }

    let expand = quote! {
        #parsed

        #(#outer_impls)*
    };

    Ok(expand.into())
}

pub fn module(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);

    match _module(parsed) {
        Ok(tk) => tk,
        Err(e) => e.to_compile_error().into()
    }
}
