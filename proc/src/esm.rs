
use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{parse::{Parse, ParseBuffer}, punctuated::Punctuated, token::{Comma, Plus, Semi}, *};

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

        let field_idens: Vec<LitByteStr> = fields.iter().flat_map(|f| f.idens).collect();
        let field_names: Vec<Ident> = fields.iter().flat_map(|f| f.get_names()).collect();
        let field_types: Vec<Type> = fields.iter().flat_map(|f| f.get_types()).collect();

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


pub struct FieldDefinition {
    pub required: bool,
    pub idens: Vec<LitByteStr>,
    pub names: Vec<Ident>,
    pub field_types: Vec<Type>,
}

impl parse::Parse for FieldDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let required: bool = if input.peek(Token![+]) {
            let required: Token![+] = input.parse()?;
            true
        } else {
            false
        };

        // Check if the field is a common field
        let mut idens: Vec<LitByteStr> = Vec::new();
        let mut names: Vec<Ident> = Vec::new();
        let mut field_types: Vec<Type> = Vec::new();
        if input.peek(LitByteStr) {
            let ident: LitByteStr = input.parse()?;
            input.parse::<Token![,]>()?;
            let name: Ident = input.parse()?;
            input.parse::<Token![,]>()?;
            let field_type: Type = input.parse()?;
            idens.push(ident);
            names.push(name);
            field_types.push(field_type);
        } else {
            let map = common_map();
            let name: Ident = input.parse()?;
            let mut common = common_field(&name, &map);
            let fd = syn::parse2::<Vec<FieldDefinition>>(common.clone()).unwrap();

            //println!("Parsed common field {}: {:?}", name, parsed);
            for field in fd {
                idens.push(field.idens[0].clone());
                names.push(field.names[0].clone());
                field_types.push(field.field_types[0].clone());
            }
        }


        Ok(FieldDefinition { required, idens, names, field_types })
    }
}


pub fn common_map() -> HashMap<[u8; 4], TokenStream> {
    let mut map = HashMap::new();
    map.insert(*b"EDID", quote! { b"EDID", EditorId, EditorId });
    map.insert(*b"OBND", quote! { b"OBND", ObjectBounds, ObjectBounds });
    map.insert(*b"MODL", quote! { b"MODL", ModelPath, ModelPath });
    map.insert(*b"MODT", quote! { b"MODT", ModelTexture, ModelTexture });
    map.insert(*b"MODC", quote! { b"MODC", ModelColorMap, ModelColorMap });
    map.insert(*b"MODS", quote! { b"MODS", ModelMaterialSwap, ModelMaterialSwap });
    map.insert(*b"MODF", quote! { b"MODF", ModelFlags, ModelFlags });
    map
}


pub fn common_field(iden: &Ident, map: &HashMap<[u8; 4], TokenStream>) -> TokenStream {
    match iden.to_string().as_str() {
        "EditorId" =>           map.get(b"EDID").unwrap().clone(),
        "ObjectBounds" =>       map.get(b"OBND").unwrap().clone(),
        "ModelPath" =>          map.get(b"MODL").unwrap().clone(),
        "ModelTexture" =>       map.get(b"MODT").unwrap().clone(),
        "ModelMaterialSwap" =>  map.get(b"MODS").unwrap().clone(),
        "ModelColorMap" =>      map.get(b"MODC").unwrap().clone(),
        "ModelFlags" =>         map.get(b"MODF").unwrap().clone(),
        "AllModelData" => {
            let mut all = TokenStream::new();
            all.extend(map.get(b"MODL").unwrap().clone());
            all.extend(map.get(b"MODT").unwrap().clone());
            all.extend(map.get(b"MODC").unwrap().clone());
            all.extend(map.get(b"MODS").unwrap().clone());
            all.extend(map.get(b"MODF").unwrap().clone());
            all = quote! { [#all] };
            all
        },
        _ => {
            unimplemented!("Common field {} not implemented", iden);
        }
    }
}