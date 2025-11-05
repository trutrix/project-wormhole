
use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{*, parse::Parse, punctuated::Punctuated};


// ====================================================================================================

pub struct RecordDefinition {
    pub _iden: LitByteStr,
    pub name: Ident,
    pub fields: Punctuated<FieldDefinition, Token![;]>
}

impl Parse for RecordDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let iden: LitByteStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner;
        bracketed!(inner in input);
        let fields = inner.parse_terminated(FieldDefinition::parse, Token![;])?;

        Ok(RecordDefinition { _iden: iden, name, fields })
    }
}

impl ToTokens for RecordDefinition {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        //let iden = &self.iden;
        let name = &self.name;
        let name_field = Ident::new(format!("{}Field", name.clone().to_string().as_str()).as_str(), name.span());
        //let name_test1 = Ident::new(format!("{}Test", name.clone().to_string().as_str()).as_str(), name.span());
        
        let fields = &self.fields;

        let field_idens: Vec<LitByteStr> = fields.iter().flat_map(|f| f.idens.clone()).collect();
        let field_names: Vec<Ident> = fields.iter().flat_map(|f| f.names.clone()).collect();
        let field_types: Vec<Type> = fields.iter().flat_map(|f| f.field_types.clone()).collect();

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

// ====================================================================================================

pub struct FieldDefinition {
    pub _required: bool, // Unused (for now) >:)
    pub idens: Vec<LitByteStr>,
    pub names: Vec<Ident>,
    pub field_types: Vec<Type>,
}

impl parse::Parse for FieldDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {

        // Check if the field is required
        let required = if input.peek(Token![+]) {
            input.parse::<Token![+]>()?;
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
            let common = common_map();
            let name: Ident = input.parse()?;
            
            if let Some(fd) = common.get(&name.to_string()) {
                idens.extend(fd.idens.clone());
                names.extend(fd.names.clone());
                field_types.extend(fd.field_types.clone());
            } else {
                return Err(syn::Error::new(name.span(), format!("Unknown common field: {}", name)));
            }

        }


        Ok(FieldDefinition { _required: required, idens, names, field_types })
    }
}

// ====================================================================================================

use super::consts::*;

pub fn common_map() -> HashMap<String, FieldDefinition> {
    let mut map = HashMap::new();
    map.insert(EDID_NAME.to_string(), {
        FieldDefinition {
            _required: true,
            idens: vec![LitByteStr::new(EDID_CODE, proc_macro2::Span::call_site())],
            names: vec![Ident::new(EDID_NAME, proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str(EDID_TYPE).unwrap()],
        }
    });

    map.insert(DESC_NAME.to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![LitByteStr::new(DESC_CODE, proc_macro2::Span::call_site())],
            names: vec![Ident::new(DESC_NAME, proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str(DESC_TYPE).unwrap()],
        }
    );

    map.insert(CTDA_NAME.to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![
                LitByteStr::new(CTDA_CODE, proc_macro2::Span::call_site()),
                LitByteStr::new(CIS1_CODE, proc_macro2::Span::call_site()),
                LitByteStr::new(CIS2_CODE, proc_macro2::Span::call_site())
            ],
            names: vec![
                Ident::new(CTDA_NAME, proc_macro2::Span::call_site()),
                Ident::new(CIS1_NAME, proc_macro2::Span::call_site()),
                Ident::new(CIS2_NAME, proc_macro2::Span::call_site())
            ],
            field_types: vec![
                syn::parse_str(CTDA_TYPE).unwrap(),
                syn::parse_str(CIS1_TYPE).unwrap(),
                syn::parse_str(CIS2_TYPE).unwrap(),
            ],
        }
    );
    
    map.insert(OBND_NAME.to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![LitByteStr::new(OBND_CODE, proc_macro2::Span::call_site())],
            names: vec![Ident::new(OBND_NAME, proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str(OBND_TYPE).unwrap()],
        }
    );

    map.insert(PTRN_NAME.to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![LitByteStr::new(PTRN_CODE, proc_macro2::Span::call_site())],
            names: vec![Ident::new(PTRN_NAME, proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str(PTRN_TYPE).unwrap()],
        }
    );

    map.insert(KYWD_NAME.to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![LitByteStr::new(KYWD_CODE, proc_macro2::Span::call_site()), LitByteStr::new(KSIZ_CODE, proc_macro2::Span::call_site())],
            names: vec![Ident::new(KYWD_NAME, proc_macro2::Span::call_site()), Ident::new(KSIZ_NAME, proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str(KYWD_TYPE).unwrap(), syn::parse_str(KSIZ_TYPE).unwrap()],
        }
    );

    map.insert(VMAD_NAME.to_string(), 
    FieldDefinition { 
        _required: false,
        idens: vec![LitByteStr::new(VMAD_CODE, proc_macro2::Span::call_site())],
        names: vec![Ident::new(VMAD_NAME, proc_macro2::Span::call_site())],
        field_types: vec![syn::parse_str(VMAD_TYPE).unwrap()],
    });


    map.insert(FULL_NAME.to_string(), 
    
        FieldDefinition {
            _required: true,
            idens: vec![LitByteStr::new(FULL_CODE, proc_macro2::Span::call_site())],
            names: vec![Ident::new(FULL_NAME, proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str(FULL_TYPE).unwrap()],
        }
    );

    map.insert("ModelData".to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![
                LitByteStr::new(b"MODL", proc_macro2::Span::call_site()),
                LitByteStr::new(b"MODT", proc_macro2::Span::call_site()),
                LitByteStr::new(b"MODC", proc_macro2::Span::call_site()),
                LitByteStr::new(b"MODS", proc_macro2::Span::call_site()),
                LitByteStr::new(b"MODF", proc_macro2::Span::call_site()),
            ],
            names: vec![
                Ident::new("ModelPath", proc_macro2::Span::call_site()),
                Ident::new("ModelTexture", proc_macro2::Span::call_site()),
                Ident::new("ModelColorMap", proc_macro2::Span::call_site()),
                Ident::new("ModelMaterialSwap", proc_macro2::Span::call_site()),
                Ident::new("ModelFlags", proc_macro2::Span::call_site()),
            ],
            field_types: vec![
                syn::parse_str("ModelPath").unwrap(),
                syn::parse_str("ModelTexture").unwrap(),
                syn::parse_str("ModelColorMap").unwrap(),
                syn::parse_str("ModelMaterialSwap").unwrap(),
                syn::parse_str("ModelFlags").unwrap(),
            ],
        }
    );

    map.insert("Destructible".to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![
                LitByteStr::new(b"DEST", proc_macro2::Span::call_site()),
                LitByteStr::new(b"DSTD", proc_macro2::Span::call_site()),
                LitByteStr::new(b"DSTF", proc_macro2::Span::call_site()),
                LitByteStr::new(b"DMDL", proc_macro2::Span::call_site()),
                LitByteStr::new(b"DMDT", proc_macro2::Span::call_site()),
            ],
            names: vec![
                Ident::new("Destructible", proc_macro2::Span::call_site()),
                Ident::new("DestructibleStageData", proc_macro2::Span::call_site()),
                Ident::new("DestructibleEnd", proc_macro2::Span::call_site()),
                Ident::new("DestructibleModelPath", proc_macro2::Span::call_site()),
                Ident::new("DestructibleModelTexture", proc_macro2::Span::call_site()),
            ],
            field_types: vec![
                syn::parse_str("DestructibleHeader").unwrap(),
                syn::parse_str("Vec<DestructibleStage>").unwrap(),
                syn::parse_str("EmptyParser").unwrap(),
                syn::parse_str("ESMString").unwrap(),
                syn::parse_str("ModelTexture").unwrap(),
            ],
        }
    );

    map.insert("Properties".to_string(), 
        FieldDefinition {
            _required: false,
            idens: vec![LitByteStr::new(b"PRPS", proc_macro2::Span::call_site())],
            names: vec![Ident::new("Properties", proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str("RecordProperty").unwrap()],
        }
    );

    map.insert("PickUpPutDown".to_string(),
        FieldDefinition {
            _required: false,
            idens: vec![LitByteStr::new(b"YNAM", proc_macro2::Span::call_site()),LitByteStr::new(b"ZNAM", proc_macro2::Span::call_site())],
            names: vec![Ident::new("PickUpSound", proc_macro2::Span::call_site()),Ident::new("PutDownSound", proc_macro2::Span::call_site())],
            field_types: vec![syn::parse_str("FormId").unwrap(),syn::parse_str("FormId").unwrap()],
        }
    );

    map
}