
use syn::{punctuated::Punctuated, *};

pub struct RecordDefinition {
    pub iden: LitByteStr,
    pub name: Ident,
    pub fields: Punctuated<FieldDefinition, Token![;]>, 
}

impl syn::parse::Parse for RecordDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let iden: LitByteStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner;
        bracketed!(inner in input);
        let fields: Punctuated<FieldDefinition, Token![;]> = inner.parse_terminated(FieldDefinition::parse, Token![;])?;
        Ok(RecordDefinition { iden, name, fields })
    }
}

#[derive(syn_derive::Parse)]
pub struct FieldDefinition {
    pub iden: LitByteStr,
    pub comma1: Token![,],
    pub name: Ident,
    pub comma2: Token![,],
    pub field_type: Type,
}


// impl syn::parse::Parse for RecordDefinition {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         Ok(RecordDefinition {})
//     }
// }