use std::collections::BTreeSet;

use syn::{parse_quote, Ident, Item, ItemStruct, Result, FieldValue, ItemImpl};

use crate::utils::generics_to_ident_list;

pub fn impl_default(ctx_name: Ident, metadata: Ident, storage_names: &BTreeSet<Option<Ident>>, st: &ItemStruct) -> Result<Item> {
    let mut field = Vec::new();

    for item in &st.fields {
        let md = metadata.clone();

        if item.ident == Some(md.clone()) {
            let f: FieldValue = parse_quote! {
                #md: Self::metadata()
            };

            field.push(f);
        } else if item.ident == Some(ctx_name.clone()) {}
        else if storage_names.contains(&item.ident) {
            let ident = item.ident.clone();

            let f: FieldValue = parse_quote! {
                #ident: baseline::bs3::Storage::new(___ctx.store(), baseline::bs3::prelude::Merkle::new(___ctx.digest()))
            };

            field.push(f);
        }
        else {
            let ident = item.ident.clone();

            let f: FieldValue = parse_quote! {
                #ident: Default::default()
            };

            field.push(f);
        }
    }

    let ident = &st.ident;
    let generics_params = generics_to_ident_list(&st.generics);

    let mut res: ItemImpl = parse_quote! {
        impl #ident<#generics_params> {
            fn new(___ctx: C) -> Self
            where C: baseline::prelude::ContextSetable,
            {
                use baseline::prelude::ModuleMetadata;

                Self {
                    #(#field,)*
                    #ctx_name: ___ctx,
                }
            }
        }
    };

    res.generics = st.generics.clone();

    Ok(Item::Impl(res))
}
