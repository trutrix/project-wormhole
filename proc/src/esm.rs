
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse::Parse, punctuated::Punctuated, token::{Comma, Plus}, *};

pub struct RecordDefinition {
    pub iden: LitByteStr,
    pub name: Ident,
    pub fields: Punctuated<FieldDefinition, Token![;]>
}

impl syn::parse::Parse for RecordDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let iden: LitByteStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner;
        bracketed!(inner in input);
        let fields = inner.parse_terminated(FieldDefinition::parse, Token![;])?;

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

        let field_idens: Vec<_> = fields.iter().map(|f| f.get_iden()).collect();
        let field_names: Vec<_> = fields.iter().map(|f| f.get_name()).collect();
        let field_types: Vec<_> = fields.iter().map(|f| f.get_type()).collect();

        let field_otypes: Vec<_> = fields.iter().map(|f| {
            if f.is_required().is_some() { 
                
                let ft = f.get_type();
                quote! { #ft}
            } else {
                let ft = f.get_type();
                
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

                    match value.data {
                        RawRecordData::Pointer(items) => {
                            let (_, fields) = many0(complete(#name_field::parse))(items).expect("Failed to convert RawRecord to #name");
                            Ok(Self { header: value.header, fields })
                        },
                        RawRecordData::Decompressed(items) => {
                            let (_, fields) = many0(complete(#name_field::parse))(items.as_ref()).expect("Failed to convert RawRecord to #name");
                            Ok(Self { header: value.header, fields })
                        }
                    }
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
                            //unimplemented!("Field {} not implemented", header.iden());
                            Ok((i, #name_field::Unknown(header.iden().clone())))
                        }
                    }

                    
                }
            }

        });
    }
}

#[derive(syn_derive::Parse, Clone)]
pub struct FieldDefinitionCustom {
    pub required: Option<Token![+]>,
    pub iden: LitByteStr,
    pub _c1: Token![,],
    pub name: Ident,
    pub _c2: Token![,],
    pub field_type: Type,
}


#[derive(Clone)]
pub enum FieldDefinition {
    Common(Ident),
    Custom(FieldDefinitionCustom),
}

impl Parse for FieldDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        if input.peek(syn::LitByteStr) {
            let out = input.parse::<FieldDefinitionCustom>()?;
            Ok(FieldDefinition::Custom(out))
        } else {
            if let Ok(out) = input.parse::<Ident>() {
                Ok(FieldDefinition::Common(out))
            } else {
                panic!("Ident could not be parsed.")
            }
            
        }
    }
}

impl FieldDefinition {
    fn get_iden(&self) -> LitByteStr {
        match self {
            FieldDefinition::Common(custom) => {
                common_field(custom).iden.clone()
            },
            FieldDefinition::Custom(f) => f.iden.clone(),
        }
    }

    fn get_name(&self) -> Ident {
        match self {
            FieldDefinition::Common(custom) => {
                common_field(custom).name.clone()
            },
            FieldDefinition::Custom(f) => f.name.clone(),
        }
    }

    fn get_type(&self) -> Type {
        match self {
            FieldDefinition::Common(custom) => {
                common_field(custom).field_type.clone()
            },
            FieldDefinition::Custom(f) => f.field_type.clone(),
        }
    }

    fn is_required(&self) -> Option<Plus> {
        match self {
            FieldDefinition::Common(_) => None,
            FieldDefinition::Custom(f) => f.required,
        }
    }
}




/// TODO: This is a temporary solution. We should have a better way to define common fields.
pub fn common_field(input: &Ident) -> FieldDefinitionCustom {


    match input.to_string().as_str() {
        "EditorId" => {
            let t = quote! { b"EDID", EditorId, ESMString };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        "ObjectBounds" => {
            let t = quote! { b"OBND", ObjectBounds, ObjectBounds };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        "ModelPath" => {
            let t = quote! { b"MODL", ModelPath, ModelPath };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        "ModelTexture" => {
            let t = quote! { b"MODT", ModelTexture, ModelTexture };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        "ModelColorMap" => {
            let t = quote! { b"MODC", ModelColorMap, ModelColorMap };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        "ModelMaterialSwap" => {
            let t = quote! { b"MODS", ModelMaterialSwap, ModelMaterialSwap };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        "ModelFlags" => {
            let t = quote! { b"MODF", ModelFlags, ModelFlags };
            let fd = syn::parse2(t).unwrap();
            fd
        }
        _ => unimplemented!("Common field {:?} not implemented", input),
    }
}