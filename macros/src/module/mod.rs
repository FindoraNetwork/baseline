mod clone;
mod ctx;
mod default;
mod metadata;
mod storage;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, Error, Fields, ItemStruct, Result};

use core::panic;
use std::{collections::BTreeSet, mem};

pub fn _module(mut parsed: ItemStruct) -> Result<TokenStream> {
    let mut outer_impls = Vec::new();

    let mut ctx_type = None;
    let mut ctx_name = None;
    let mut metadata_name = None;

    let mut storage_names = BTreeSet::new();

    if let Fields::Named(n) = &mut parsed.fields {
        for field in &mut n.named {
            let attrs = mem::take(&mut field.attrs);

            for attr in attrs {
                let path = attr
                    .path
                    .get_ident()
                    .ok_or_else(|| Error::new(Span::call_site(), "read attr failed."))?
                    .to_string();

                match path.as_str() {
                    "context" => {
                        let ctx = ctx::impl_ctx(field.clone(), &parsed.ident, &parsed.generics);

                        ctx_type = Some(field.ty.clone());
                        ctx_name = field.ident.clone();
                        outer_impls.push(ctx);
                    }
                    "metadata" => {
                        let item = metadata::impl_metadata(attr, &parsed.ident, &parsed.generics)?;

                        metadata_name = field.ident.clone();
                        outer_impls.push(item);
                    }
                    "storage" => {
                        let ty = ctx_type.clone().ok_or_else(|| {
                            Error::new(Span::call_site(), "Context must be set first than storage")
                        })?;

                        storage::impl_storage(ty, field, attr)?;

                        storage_names.insert(field.ident.clone());
                    }
                    "dependence" => {}
                    _ => panic!("no {} support, pleasr remove it", path),
                }
            }
        }
    }

    let metadata_name =
        metadata_name.ok_or_else(|| Error::new(Span::call_site(), "metadata must be defined"))?;

    let ctx_name =
        ctx_name.ok_or_else(|| Error::new(Span::call_site(), "context must be defined"))?;

    let _default = default::impl_default(
        ctx_name.clone(),
        metadata_name.clone(),
        &storage_names,
        &parsed,
    )?;
    let _clone = clone::impl_clone(&parsed)?;

    let expand = quote! {
        #parsed

        #_default

        #_clone

        #(#outer_impls)*
    };

    Ok(expand.into())
}

pub fn module(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);

    match _module(parsed) {
        Ok(tk) => tk,
        Err(e) => e.to_compile_error().into(),
    }
}
