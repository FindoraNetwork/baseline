use proc_macro2::Span;
use syn::{punctuated::Punctuated, Error, Lit, MetaNameValue, Result, Token, Type};

pub struct Attr {
    pub context: Type,
}

impl Attr {
    pub fn from_meta(m: Punctuated<MetaNameValue, Token![,]>) -> Result<Self> {
        let mut context = None;

        for item in m {
            if let Some(name) = item.path.get_ident() {
                if name.to_string() == "context" {
                    if let Lit::Str(s) = item.lit {
                        let ty: Type = s.parse()?;
                        context = Some(ty);
                    }
                }
            }
        }

        Ok(Self {
            context: context.ok_or(Error::new(Span::call_site(), "Must set context"))?,
        })
    }
}
