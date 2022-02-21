use syn::{parse_quote, Field, Generics, Ident, Item, ItemImpl};

use crate::utils::generics_to_ident_list;

pub fn impl_ctx(field: Field, ident: &Ident, generics: &Generics) -> Item {
    let type_ = field.ty;
    let field_name = field.ident;

    let generics_params = generics_to_ident_list(generics);

    let mut impl_: ItemImpl = parse_quote! {
        impl baseline::prelude::Module for #ident<#generics_params> {
            type Context = #type_;

            fn set_ctx(&mut self, context: Self::Context) {
                self.#field_name = context;
            }
        }
    };

    impl_.generics = generics.clone();

    Item::Impl(impl_)
}
