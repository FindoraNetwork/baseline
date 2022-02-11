use syn::{parse_quote, Ident, Item, ItemStruct, Result};

pub fn impl_default(_metadata: Ident, _st: &ItemStruct) -> Result<Item> {
    Ok(parse_quote!(
        mod __aa {}
    ))
}
