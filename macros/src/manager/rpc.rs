use syn::{ItemStruct, Item, Result, ItemImpl, parse_quote};

use crate::utils::generics_to_ident_list;

pub fn impl_rpc(st: &ItemStruct) -> Result<Item> {
    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        #[baseline::async_trait]
        impl baseline::prelude::RPC for #ident<#generics_params> {
            async fn call(&mut self, req: baseline::types::rpc::Request) -> baseline::types::rpc::Response {

                match req.method.as_str() {
                    // #(#method_names => #call_bodys,)*
                    _ => baseline::types::rpc::Response::not_found()
                }
            }
        }
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
