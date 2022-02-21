use proc_macro2::Span;
use syn::{parse_quote, Attribute, Error, Field, Lit, Meta, NestedMeta, Result, Type};

pub fn impl_storage(ty: Type, field: &mut Field, attr: Attribute) -> Result<()> {
    let type_ = field.ty.clone();

    let arg = attr.parse_meta()?;

    let merkle_type: Type = match arg {
        Meta::Path(_) => parse_quote!(baseline::bs3::merkle::EmptyMerkle),
        Meta::List(l) => {
            let n = l
                .nested
                .first()
                .ok_or_else(|| Error::new(Span::call_site(), "Internal error."))?;
            if let NestedMeta::Meta(m) = n {
                if let Meta::NameValue(v) = m {
                    let name = v
                        .path
                        .get_ident()
                        .ok_or_else(|| Error::new(Span::call_site(), "Must set key"))?;

                    if *name == "merkle" {
                        if let Lit::Str(s) = &v.lit {
                            s.parse()?
                        } else {
                            return Err(Error::new(Span::call_site(), "Type must be string"));
                        }
                    } else {
                        return Err(Error::new(Span::call_site(), "Must set key is merkle"));
                    }
                } else {
                    return Err(Error::new(Span::call_site(), "Must set key is merkle"));
                }
            } else {
                return Err(Error::new(Span::call_site(), "Must set merkle type"));
            }
        }
        _ => return Err(Error::new(Span::call_site(), "Only support path and list")),
    };

    let final_type = parse_quote!(baseline::bs3::Storage<#type_, #merkle_type<#ty::Digest>, #ty::Store>);

    field.ty = final_type;

    Ok(())
}
