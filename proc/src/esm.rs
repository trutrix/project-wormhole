
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
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

impl ToTokens for RecordDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let iden = &self.iden;
        let name = &self.name;
        let name_field = Ident::new(format!("{}Field", name.clone().to_string().as_str()).as_str(), name.span());
        let name_test1 = Ident::new(format!("{}Test", name.clone().to_string().as_str()).as_str(), name.span());
        let fields = &self.fields;
        let field_idens: Vec<_> = fields.iter().map(|f| &f.iden).collect();
        let field_names: Vec<_> = fields.iter().map(|f| &f.name).collect();
        let field_types: Vec<_> = fields.iter().map(|f| &f.field_type).collect();
        let field_otypes: Vec<_> = fields.iter().map(|f| {
            if f.required.is_some() { 
                
                let ft = f.field_type.clone();
                quote! { #ft}
            } else {
                let ft = f.field_type.clone();
                
                quote! { Option<#ft> }
            }
        }).collect();
        tokens.extend(quote! {
            #[derive(Debug)]
            pub struct #name {
                pub header: RecordHeader,
                pub fields: Vec<#name_field>
            }

            impl Parse<&[u8]> for #name {
                fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
                    let (i, (header, data)) = alloc_record(i)?;
                    let (_, fields) = many0(complete(#name_field::parse_le))(data)?;
                    Ok((i, Self { header, fields }))
                }
            }


            impl TryFrom<RawRecord<'_>> for #name {
                type Error = crate::esm::ESMError;

                fn try_from(value: RawRecord<'_>) -> Result<Self, Self::Error> {
                    let (_, fields) = many0(complete(#name_field::parse))(value.data).expect("Failed to convert RawRecord to #name");
                    Ok(Self { header: value.header, fields })
                }
            }
            

            #[derive(Debug)] 
            pub enum #name_field {
                Unknown(FourCC),
                #(#field_names(#field_types)),*
            }


            impl Parse<&[u8]> for #name_field {
                fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
                    let (i, (header, data)) = alloc_field(i)?;
                    match &header.iden().0 {
                        #(
                            #field_idens => {
                                let (_, out) = <#field_types>::parse_le(data)?;
                                Ok((i, Self::#field_names(out)))
                            }
                        )*
                        _ => {
                            unimplemented!("Field {} not implemented", header.iden());
                            Ok((i, #name_field::Unknown(header.iden().clone())))
                        }
                    }

                    
                }
            }

        });
    }
}

#[derive(syn_derive::Parse)]
pub struct FieldDefinition {
    pub required: Option<Token![+]>,
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