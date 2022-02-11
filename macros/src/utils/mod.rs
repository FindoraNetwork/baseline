use std::mem;
use syn::{punctuated::Punctuated, GenericParam, Generics, Token};

pub fn generics_to_ident_list(generics: &Generics) -> Punctuated<GenericParam, Token![,]> {
    let mut v = generics.params.clone();

    for gp in &mut v {
        if let GenericParam::Type(e) = gp {
            let _ = mem::take(&mut e.bounds);
        }
    }

    v
}
