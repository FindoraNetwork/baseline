use syn::{Result, parse_quote, Item, Ident, ItemStruct};

pub fn impl_default(metadata: Ident, st: &ItemStruct) -> Result<Item> {


    Ok(parse_quote!(mod __aa {}))
}
