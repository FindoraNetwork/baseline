use core::mem;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, Block, Error, ImplItem, ItemImpl, Lit, LitStr, Meta, MetaList,
    NestedMeta, Path, Result, Signature,
};

fn check_path_rpc(path: &Path) -> Result<()> {
    if *path
        .get_ident()
        .ok_or_else(|| Error::new(Span::call_site(), "get ident error"))?
        == "rpc"
    {
        Ok(())
    } else {
        Err(Error::new(Span::call_site(), "rpc attr must be rpc"))
    }
}

fn parse_rpc_meta_name(ml: MetaList) -> Result<Lit> {
    let fm = ml
        .nested
        .first()
        .ok_or_else(|| Error::new(Span::call_site(), "no name for rpc"))?;
    if let NestedMeta::Meta(m) = fm {
        if let Meta::NameValue(m) = m {
            match m
                .path
                .get_ident()
                .ok_or_else(|| Error::new(Span::call_site(), "Muse be ident"))?
                .to_string()
                .as_str()
            {
                "name" => Ok(m.lit.clone()),
                _ => Err(Error::new(Span::call_site(), "Only support name")),
            }
        } else {
            Err(Error::new(Span::call_site(), "expect meta name value"))
        }
    } else {
        Err(Error::new(Span::call_site(), "expect meta"))
    }
}

fn build_call(sig: &Signature) -> Result<Block> {
    let nnn = &sig.ident;

    // TODO: check signature for rpc.

    let arg_len = sig.inputs.len();

    if arg_len == 1 {
        Ok(parse_quote!({
            let resp = self.#nnn().await;

            let r = baseline::rpc::helpers::to_result_response(resp);

            baseline::rpc::helpers::to_response(r)
        }))
    } else if arg_len == 2 {
        Ok(parse_quote!( {
            match baseline::prelude::Requester::request(req) {
                Ok(e) => {
                    let resp = self.#nnn(e).await;

                    let r = baseline::rpc::helpers::to_result_response(resp);

                    baseline::rpc::helpers::to_response(r)
                },
                Err(e) => e.to_response()
            }

        }))
    } else {
        Err(Error::new(
            Span::call_site(),
            "format error for rpc siganture",
        ))
    }
}

pub fn _main(mut input: ItemImpl) -> Result<TokenStream> {
    let mut method_names = Vec::new();
    let mut call_bodys = Vec::new();

    for method in &mut input.items {
        if let ImplItem::Method(m) = method {
            let attrs = mem::take(&mut m.attrs);

            for attr in attrs {
                let args = attr.parse_meta()?;

                match args {
                    Meta::Path(p) => {
                        check_path_rpc(&p)?;
                        let method_name = LitStr::new(&m.sig.ident.to_string(), Span::call_site());

                        method_names.push(Lit::Str(method_name));

                        let call_body = build_call(&m.sig)?;

                        call_bodys.push(call_body);
                    }
                    Meta::List(l) => {
                        check_path_rpc(&l.path)?;
                        let method_name = parse_rpc_meta_name(l)?;

                        method_names.push(method_name);

                        let call_body = build_call(&m.sig)?;

                        call_bodys.push(call_body);
                    }
                    _ => return Err(Error::new(Span::call_site(), "format error.")),
                }
            }
        }
    }

    let ident = *input.self_ty.clone();

    let mut m: ItemImpl = parse_quote! {
        #[baseline::async_trait]
        impl baseline::prelude::RPC for #ident {
            async fn call(&mut self, req: baseline::types::rpc::Request) -> baseline::types::rpc::Response {

                match req.method.as_str() {
                    #(#method_names => #call_bodys,)*
                    _ => baseline::types::rpc::Response::not_found()
                }
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
