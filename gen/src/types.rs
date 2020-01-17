use anyhow::Result;
use heck::CamelCase;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use witx::*;

pub fn gen(doc: Document) -> Result<TokenStream> {
    let mut output = TokenStream::new();
    for namedtype in doc.typenames() {
        let def = match &namedtype.tref {
            TypeRef::Value(v) => typename_def(&namedtype.name, v)?,
            TypeRef::Name(nt) => typename_alias(&namedtype.name, nt)?,
        };
        output.extend(def);
    }

    Ok(output)
}

fn typename_alias(name: &Id, ty: &NamedType) -> Result<TokenStream> {
    let _ = name;
    let _ = ty;
    unimplemented!()
}

fn typename_def(name: &Id, ty: &Type) -> Result<TokenStream> {
    let ident = id_to_typename(name);
    let def = match ty {
        Type::Enum(e) => {
            let cases: TokenStream = e
                .variants
                .iter()
                .map(|v| format_ident!("{}", v.name.as_str().to_camel_case()))
                .map(|i| quote!(#i,))
                .collect();
            let repr = intrepr_to_type(e.repr);
            quote! {
                #[repr(#repr)]
                #[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Display, Debug)]
                pub enum #ident { #cases }
            }
        }
        _ => unimplemented!(),
    };
    Ok(def)
}

fn id_to_typename(id: &Id) -> Ident {
    format_ident!("{}", id.as_str().to_camel_case())
}

fn intrepr_to_type(i: IntRepr) -> TokenStream {
    match i {
        IntRepr::U8 => quote!(u8),
        IntRepr::U16 => quote!(u16),
        IntRepr::U32 => quote!(u32),
        IntRepr::U64 => quote!(u64),
    }
}
