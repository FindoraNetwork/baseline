use syn::{parse_quote, Item, ItemImpl, ItemStruct, Result};

use crate::utils::generics_to_ident_list;

pub fn impl_block(st: &ItemStruct) -> Result<Item> {
    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        #[baseline::async_trait]
        impl baseline::prelude::Block for #ident<#generics_params> {
            async fn apply_txs(&mut self, _tx: &[Self::Transaction]) -> ExecResults
            where
                Self::Context: ContextMut,
            {
                Default::default()
            }
        }
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
