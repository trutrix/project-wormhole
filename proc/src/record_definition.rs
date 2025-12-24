#![allow(dead_code)]
use core::panic;
use std::collections::HashMap;

use proc_macro2::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::parse::discouraged::Speculative;
use syn::{*, parse::Parse, punctuated::Punctuated};

use super::record_consts::*;

// ====================================================================================================

// pub struct RecordDefinition {
//     pub _iden: LitByteStr,
//     pub name: Ident,
//     pub fields: Punctuated<FieldDefinition, Token![;]>,
//     pub flags: Punctuated<FlagDefinition, Token![;]>
// }

// impl Parse for RecordDefinition {
//     fn parse(input: parse::ParseStream) -> Result<Self> {
//         let iden: LitByteStr = input.parse()?;
//         input.parse::<Token![,]>()?;
//         let name: Ident = input.parse()?;
//         input.parse::<Token![,]>()?;
//         let inner;
//         bracketed!(inner in input);
//         let fields = inner.parse_terminated(FieldDefinition::parse, Token![;])?;

//         if input.peek(Token![,]) {
//             input.parse::<Token![,]>()?;
//             let inner2;
//             bracketed!(inner2 in input);
//             let flags = inner2.parse_terminated(FlagDefinition::parse, Token![;])?;
//             Ok(RecordDefinition { _iden: iden, name, fields, flags })
//         } else {
//             Ok(RecordDefinition { _iden: iden, name, fields, flags: Punctuated::new() })
//         }
//     }
// }

// impl ToTokens for RecordDefinition {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         //let iden = &self.iden;
//         let name = &self.name;
//         let name_field = Ident::new(format!("{}Field", name.clone().to_string().as_str()).as_str(), name.span());
//         //let name_test1 = Ident::new(format!("{}Test", name.clone().to_string().as_str()).as_str(), name.span());
        
//         let fields = &self.fields;

//         let field_idens: Vec<LitByteStr> = fields.iter().flat_map(|f| f.idens.clone()).collect();
//         let field_names: Vec<Ident> = fields.iter().flat_map(|f| f.names.clone()).collect();
//         let field_types: Vec<Type> = fields.iter().flat_map(|f| f.field_types.clone()).collect();

//         tokens.extend(quote! {
//             #[derive(Debug)]
//             pub struct #name {
//                 pub header: RecordHeader,
//                 pub fields: Vec<#name_field>
//             }

//             // impl Parse<&[u8]> for #name {
//             //     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
//             //         let (i, (header, data)) = alloc_record(i)?;
//             //         let (_, fields) = many0(#name_field::parse_le)(data)?;
//             //         Ok((i, Self { header, fields }))
//             //     }
//             // }

//             impl Parse<&[u8]> for #name {
//                 fn parse(i: &[u8]) -> IResult<&[u8], Self> {
//                     let (i, (header, raw)) = alloc_record(i)?;

//                     if header.flags.is_compressed() {
//                         if let Ok(dec) = decompress_record(raw) {
                            
//                             if let Ok((_, fields)) = many0(#name_field::parse)(&dec) {
//                                 return Ok((i, Self { header, fields }));
//                             } else {
//                                 return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
//                             }
                            
//                         } else {
//                             panic!("Could not decompress record: {:?}", header);
//                         }
                        
//                     } else {
//                         if let Ok((_, fields)) = many0(#name_field::parse)(&raw) {
//                             return Ok((i, Self { header, fields }));
//                         } else {
//                             return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
//                         }
//                     }       
//                 }
//             }


//             impl TryFrom<RawRecord<'_>> for #name {
//                 type Error = crate::esm::ESMError;

//                 fn try_from(value: RawRecord<'_>) -> Result<Self, Self::Error> {

//                     match value.data {
//                         RawRecordData::Pointer(items) => {
//                             let (_, fields) = many0(#name_field::parse)(items).expect("Failed to convert RawRecord to #name");
//                             Ok(Self { header: value.header, fields })
//                         },
//                         RawRecordData::Decompressed(items) => {
//                             let (_, fields) = many0(#name_field::parse)(items.as_ref()).expect("Failed to convert RawRecord to #name");
//                             Ok(Self { header: value.header, fields })
//                         }
//                     }
//                 }
//             }
            

//             #[derive(Debug)] 
//             pub enum #name_field {
//                 Unknown(FourCC),
//                 #(#field_names(#field_types)),*
                
//             }


//             impl Parse<&[u8]> for #name_field {
//                 fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
//                     let (i, (header, data)) = alloc_field(i)?;
//                     match &header.iden().0 {
//                         #(
//                             #field_idens => {
//                                 let (_, out) = <#field_types>::parse_le(data)?;
//                                 Ok((i, Self::#field_names(out)))
//                             }
//                         )*
//                         _ => {
//                             //unimplemented!("Field {} not implemented", header.iden());
//                             Ok((i, #name_field::Unknown(header.iden().clone())))
//                         }
//                     }

                    
//                 }
//             }

//         });

//         for field in field_idens {
//             let v = field.value();
//             let four_cc: [u8;4] = v.try_into().unwrap();
//             match &four_cc {
//                 b"EDID" => {
//                     tokens.extend(quote! {
//                         impl RecordEditorId for #name {
//                             fn try_get_editor_id(&self) -> Option<&ESMString> {
//                                 for field in &self.fields {
//                                     match field {
//                                         #name_field::EditorId(edid) => {
//                                             return Some(edid);
//                                         },
//                                         _ => {}
//                                     }
//                                 }
//                                 None
//                             }
//                         }
//                     });
//                 }
//                 _ => {}
//             }
//         }
//     }
// }

// ====================================================================================================

pub struct RecordDefinition2 {
    pub iden: LitByteStr,
    pub name: Ident,
    pub fields: Punctuated<FieldDefinition2, Token![;]>,
    pub flags: Punctuated<FlagDefinition, Token![;]>
}

impl Parse for RecordDefinition2 {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let iden: LitByteStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let inner;
        bracketed!(inner in input);
        let fields = inner.parse_terminated(FieldDefinition2::parse, Token![;])?;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let inner2;
            bracketed!(inner2 in input);
            let flags = inner2.parse_terminated(FlagDefinition::parse, Token![;])?;
            Ok(RecordDefinition2 { iden, name, fields, flags })
        } else {
            Ok(RecordDefinition2 { iden, name, fields, flags: Punctuated::new() })
        }
    }
}

impl ToTokens for RecordDefinition2 {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let cmap = common_map2();

        let record_iden = &self.iden;
        let name = &self.name;
        let name_field = Ident::new(format!("{}Field", name.clone().to_string().as_str()).as_str(), name.span());

        let fields = &self.fields;

        let mut end_fields: Vec<&FieldDefinition2> = Vec::new();

        for field in fields {
            match &field.field_type {
                FieldType::Common(ident) => {
                    if let Some(cf) = cmap.get(&ident.to_string()) {
                        end_fields.extend(cf);
                    } else {
                        panic!("Unknown common field: {}", ident);
                    }
                }
                FieldType::Custom(_cuf) => {
                    end_fields.push(field);
                }
                FieldType::Reference(_lit_byte_strs) => {
                    end_fields.push(field);
                },
            }
        }


        let field_idens: Vec<LitByteStr> = end_fields.iter().map(|f| f.iden.clone()).collect();
        let field_names: Vec<Ident> = end_fields.iter().map(|f| f.name.clone()).collect();
        let field_types: Vec<&FieldType> = end_fields.iter().map(|f| &f.field_type).collect();
        let trait_impl = make_record_traits_impl(name, &name_field, fields);

        tokens.extend(quote! {
            #[derive(Debug)]
            pub struct #name {
                pub header: RecordHeader,
                pub fields: Vec<#name_field>
            }

            impl Parse<&[u8]> for #name {
                fn parse(i: &[u8]) -> IResult<&[u8], Self> {
                    let (i, (header, raw)) = alloc_record(i)?;

                    if &header.iden.0 != #record_iden {
                        panic!("Tried to parse {:?} record as {:?}", header, FourCC(*#record_iden))
                    }

                    if header.flags.is_compressed() {
                        if let Ok(dec) = decompress_record(raw) {
                            
                            if let Ok((_, fields)) = many0(#name_field::parse)(&dec) {
                                return Ok((i, Self { header, fields }));
                            } else {
                                return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
                            }
                            
                        } else {
                            panic!("Could not decompress record: {:?}", header);
                        }
                        
                    } else {
                        if let Ok((_, fields)) = many0(#name_field::parse)(&raw) {
                            return Ok((i, Self { header, fields }));
                        } else {
                            return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
                        }
                    }       
                }
            }

            #trait_impl
            

            impl<'esm> TryFrom<RawRecord<'esm>> for #name {
                type Error = nom::error::Error<&'esm[u8]>;

                fn try_from(value: RawRecord<'esm>) -> Result<Self, Self::Error> {

                    match value.data {
                        RawRecordData::Pointer(items) => {
                            if let Ok((_, fields)) = many0(#name_field::parse)(items) {
                                Ok(Self { header: value.header, fields })
                            } else {
                                Err(nom::error::Error::new(items, nom::error::ErrorKind::Fail))
                            }
                        },
                        RawRecordData::Decompressed(items) => {
                            if let Ok((_, fields)) = many0(#name_field::parse)(items.as_ref()) {
                                Ok(Self { header: value.header, fields })
                            } else {
                                Err(nom::error::Error::new(&[], nom::error::ErrorKind::Fail))
                            }
                        }
                    }
                }
            }

            #[derive(Debug)]
            pub enum #name_field {
                Unhandled(FourCC),
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
                            Ok((i, #name_field::Unhandled(header.iden().clone())))
                        }
                    }

                    
                }
            }
        });

        
    }
}


// ====================================================================================================

// pub struct FieldDefinition {
//     pub required: bool,
//     pub idens: Vec<LitByteStr>,
//     pub names: Vec<Ident>,
//     pub field_types: Vec<Type>,
// }

// impl parse::Parse for FieldDefinition {
//     fn parse(input: parse::ParseStream) -> Result<Self> {

//         // Check if the field is required
//         let required = if input.peek(Token![+]) {
//             input.parse::<Token![+]>()?;
//             true
//         } else {
//             false
//         };

//         // Check if the field is a common field
//         let mut idens: Vec<LitByteStr> = Vec::new();
//         let mut names: Vec<Ident> = Vec::new();
//         let mut field_types: Vec<Type> = Vec::new();
//         if input.peek(LitByteStr) {
//             let ident: LitByteStr = input.parse()?;
//             input.parse::<Token![,]>()?;
//             let name: Ident = input.parse()?;
//             input.parse::<Token![,]>()?;
//             let field_type: Type = input.parse()?;
//             idens.push(ident);
//             names.push(name);
//             field_types.push(field_type);
//         } else {
//             let common = common_map();
//             let name: Ident = input.parse()?;
            
//             if let Some(fd) = common.get(&name.to_string()) {
//                 idens.extend(fd.idens.clone());
//                 names.extend(fd.names.clone());
//                 field_types.extend(fd.field_types.clone());
//             } else {
//                 return Err(syn::Error::new(name.span(), format!("Unknown common field: {}", name)));
//             }

//         }


//         Ok(FieldDefinition { required, idens, names, field_types })
//     }
// }

// ====================================================================================================

#[derive(Clone)]
pub struct FieldDefinition2 {
    pub required: bool,
    pub iden: LitByteStr,
    pub name: Ident,
    pub field_type: FieldType,
}

impl Parse for FieldDefinition2 {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        
        let required = if input.peek(Token![+]) {
            input.parse::<Token![+]>().unwrap();
            true
        } else {
            false
        };


        // Is custom or ref field
        if input.peek(LitByteStr) {
            let iden: LitByteStr = input.parse()?;
            input.parse::<Token![,]>()?;
            let name: Ident = input.parse()?;
            input.parse::<Token![,]>()?;
            
            if input.peek(syn::token::Bracket)  {
                let orig = input.fork();
                let content;
                bracketed!(content in orig);

                if content.peek(LitByteStr) {
                    input.advance_to(&orig);
                    let refs = content.parse_terminated(LitByteStr::parse, Token![,])?;
                    Ok(FieldDefinition2 { required, iden, name, field_type: FieldType::Reference(refs.into_iter().collect()) })
                } else {
                    let field_type: Type = input.parse()?;
                    Ok(FieldDefinition2 { required, iden, name, field_type: FieldType::Custom(field_type) })
                }

                
            } else {
                let field_type: Type = input.parse()?;
                Ok(FieldDefinition2 { required, iden, name, field_type: FieldType::Custom(field_type) })
            }

        } 
        // Is common field
        // TODO: this is messy and unnecessary
        else {
            let common = common_map2();
            let name: Ident = input.parse()?;
            let ns = name.to_string();
            if let Some(fd) = common.get(&ns) {
                Ok(fd[0].clone())
                //Ok(FieldDefinition2 { required, iden: fd.idens[0].clone(), name: fd.names[0].clone(), field_type: FieldType::Common(name) })
            } else {
                return Err(syn::Error::new(name.span(), format!("Unknown common field: {}", name)));
            }
        }
    }
}

pub struct FieldDefinitionList(
    pub Vec<FieldDefinition2>
);


// ====================================================================================================

#[derive(Clone)]
pub enum FieldType {
    Common(Ident),
    Custom(Type),
    Reference(Vec<LitByteStr>)
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldType::Common(_) => {
                panic!("Common field cannot be converted directly to tokens.")
            },
            FieldType::Custom(ty) => {
                ty.to_tokens(tokens);
            },
            FieldType::Reference(_refs) => {
                quote! { FormRef }.to_tokens(tokens);
            }
        }
    }
}


// ====================================================================================================


// pub fn common_map() -> HashMap<String, FieldDefinition> {
//     let mut map = HashMap::new();
//     map.insert(EDID_NAME.to_string(), {
//         FieldDefinition {
//             required: true,
//             idens: vec![LitByteStr::new(EDID_CODE, Span::call_site())],
//             names: vec![Ident::new(EDID_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(EDID_TYPE).unwrap()],
//         }
//     });

//     map.insert(DESC_NAME.to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![LitByteStr::new(DESC_CODE, Span::call_site())],
//             names: vec![Ident::new(DESC_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(DESC_TYPE).unwrap()],
//         }
//     );

//     map.insert(CTDA_NAME.to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![
//                 LitByteStr::new(CTDA_CODE, Span::call_site()),
//                 LitByteStr::new(CIS1_CODE, Span::call_site()),
//                 LitByteStr::new(CIS2_CODE, Span::call_site())
//             ],
//             names: vec![
//                 Ident::new(CTDA_NAME, Span::call_site()),
//                 Ident::new(CIS1_NAME, Span::call_site()),
//                 Ident::new(CIS2_NAME, Span::call_site())
//             ],
//             field_types: vec![
//                 syn::parse_str(CTDA_TYPE).unwrap(),
//                 syn::parse_str(CIS1_TYPE).unwrap(),
//                 syn::parse_str(CIS2_TYPE).unwrap(),
//             ],
//         }
//     );
    
//     map.insert(OBND_NAME.to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![LitByteStr::new(OBND_CODE, Span::call_site())],
//             names: vec![Ident::new(OBND_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(OBND_TYPE).unwrap()],
//         }
//     );

//     map.insert(PTRN_NAME.to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![LitByteStr::new(PTRN_CODE, Span::call_site())],
//             names: vec![Ident::new(PTRN_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(PTRN_TYPE).unwrap()],
//         }
//     );

//     map.insert(KYWD_NAME.to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![LitByteStr::new(KYWD_CODE, Span::call_site()), LitByteStr::new(KSIZ_CODE, Span::call_site())],
//             names: vec![Ident::new(KYWD_NAME, Span::call_site()), Ident::new(KSIZ_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(KYWD_TYPE).unwrap(), syn::parse_str(KSIZ_TYPE).unwrap()],
//         }
//     );

//     map.insert(VMAD_NAME.to_string(), 
//     FieldDefinition { 
//         required: false,
//         idens: vec![LitByteStr::new(VMAD_CODE, Span::call_site())],
//         names: vec![Ident::new(VMAD_NAME, Span::call_site())],
//         field_types: vec![syn::parse_str(VMAD_TYPE).unwrap()],
//     });


//     map.insert(FULL_NAME.to_string(), 
    
//         FieldDefinition {
//             required: true,
//             idens: vec![LitByteStr::new(FULL_CODE, Span::call_site())],
//             names: vec![Ident::new(FULL_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(FULL_TYPE).unwrap()],
//         }
//     );

//     map.insert("ModelData".to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![
//                 LitByteStr::new(MODL_CODE, Span::call_site()),
//                 LitByteStr::new(MODT_CODE, Span::call_site()),
//                 LitByteStr::new(MODC_CODE, Span::call_site()),
//                 LitByteStr::new(MODS_CODE, Span::call_site()),
//                 LitByteStr::new(MODF_CODE, Span::call_site()),
//             ],
//             names: vec![
//                 Ident::new(MODL_NAME, Span::call_site()),
//                 Ident::new(MODT_NAME, Span::call_site()),
//                 Ident::new(MODC_NAME, Span::call_site()),
//                 Ident::new(MODS_NAME, Span::call_site()),
//                 Ident::new(MODF_NAME, Span::call_site()),
//             ],
//             field_types: vec![
//                 syn::parse_str(MODL_TYPE).unwrap(),
//                 syn::parse_str(MODT_TYPE).unwrap(),
//                 syn::parse_str(MODC_TYPE).unwrap(),
//                 syn::parse_str(MODS_TYPE).unwrap(),
//                 syn::parse_str(MODF_TYPE).unwrap(),
//             ],
//         }
//     );

//     map.insert("Destructible".to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![
//                 LitByteStr::new(DEST_CODE, Span::call_site()),
//                 LitByteStr::new(DSTD_CODE, Span::call_site()),
//                 LitByteStr::new(DSTF_CODE, Span::call_site()),
//                 LitByteStr::new(DMDL_CODE, Span::call_site()),
//                 LitByteStr::new(DMDT_CODE, Span::call_site()),
//                 LitByteStr::new(DSTA_CODE, Span::call_site()),
//             ],
//             names: vec![
//                 Ident::new(DEST_NAME, Span::call_site()),
//                 Ident::new(DSTD_NAME, Span::call_site()),
//                 Ident::new(DSTF_NAME, Span::call_site()),
//                 Ident::new(DMDL_NAME, Span::call_site()),
//                 Ident::new(DMDT_NAME, Span::call_site()),
//                 Ident::new(DSTA_NAME, Span::call_site()),
//             ],
//             field_types: vec![
//                 syn::parse_str(DEST_TYPE).unwrap(),
//                 syn::parse_str(DSTD_TYPE).unwrap(),
//                 syn::parse_str(DSTF_TYPE).unwrap(),
//                 syn::parse_str(DMDL_TYPE).unwrap(),
//                 syn::parse_str(DMDT_TYPE).unwrap(),
//                 syn::parse_str(DSTA_TYPE).unwrap(),
//             ],
//         }
//     );

//     map.insert(PRPS_NAME.to_string(), 
//         FieldDefinition {
//             required: false,
//             idens: vec![LitByteStr::new(PRPS_CODE, Span::call_site())],
//             names: vec![Ident::new(PRPS_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(PRPS_TYPE).unwrap()],
//         }
//     );

//     map.insert("PickUpPutDown".to_string(),
//         FieldDefinition {
//             required: false,
//             idens: vec![LitByteStr::new(YNAM_CODE, Span::call_site()),LitByteStr::new(ZNAM_CODE, Span::call_site())],
//             names: vec![Ident::new(YNAM_NAME, Span::call_site()),Ident::new(ZNAM_NAME, Span::call_site())],
//             field_types: vec![syn::parse_str(YNAM_TYPE).unwrap(),syn::parse_str(ZNAM_TYPE).unwrap()],
//         }
//     );

//     map
// }

pub fn common_map2() -> HashMap<String, Vec<FieldDefinition2>> {
    let mut map = HashMap::new();
    map.insert(EDID_NAME.to_string(),
        vec![FieldDefinition2 {
            required:true, 
            iden: LitByteStr::new(EDID_CODE, Span::call_site()), 
            name: Ident::new(EDID_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(EDID_TYPE).unwrap())
        }]
    );

    map.insert(DESC_NAME.to_string(), 
        vec![FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(DESC_CODE, Span::call_site()), 
            name: Ident::new(DESC_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(DESC_TYPE).unwrap())
        }]
    );

    map.insert(CTDA_NAME.to_string(),
        vec![
            FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(CTDA_CODE, Span::call_site()), 
            name: Ident::new(CTDA_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(CTDA_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(CIS1_CODE, Span::call_site()), 
                name: Ident::new(CIS1_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(CIS1_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(CIS2_CODE, Span::call_site()), 
                name: Ident::new(CIS2_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(CIS2_TYPE).unwrap())
            }
        ]
    );
    
    map.insert(OBND_NAME.to_string(), 
        vec![FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(OBND_CODE, Span::call_site()), 
            name: Ident::new(OBND_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(OBND_TYPE).unwrap())
        }]
    );

    map.insert(PTRN_NAME.to_string(), 
        vec![FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(PTRN_CODE, Span::call_site()), 
            name: Ident::new(PTRN_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(PTRN_TYPE).unwrap())
        }]
    );

    map.insert(KYWD_NAME.to_string(), 
        vec![
            FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(KYWD_CODE, Span::call_site()), 
            name: Ident::new(KYWD_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(KYWD_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(KSIZ_CODE, Span::call_site()), 
                name: Ident::new(KSIZ_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(KSIZ_TYPE).unwrap())
            }
        ]
    );

    map.insert(VMAD_NAME.to_string(), 
    vec![FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(VMAD_CODE, Span::call_site()), 
            name: Ident::new(VMAD_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(VMAD_TYPE).unwrap())
        }]
    );


    map.insert(FULL_NAME.to_string(), 
        vec![FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(FULL_CODE, Span::call_site()), 
            name: Ident::new(FULL_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(FULL_TYPE).unwrap())
        }]
    );

    map.insert("ModelData".to_string(), 
        vec![
            FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(MODL_CODE, Span::call_site()), 
            name: Ident::new(MODL_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(MODL_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(MODT_CODE, Span::call_site()), 
                name: Ident::new(MODT_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(MODT_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(MODC_CODE, Span::call_site()), 
                name: Ident::new(MODC_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(MODC_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(MODS_CODE, Span::call_site()), 
                name: Ident::new(MODS_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(MODS_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(MODF_CODE, Span::call_site()), 
                name: Ident::new(MODF_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(MODF_TYPE).unwrap())
            }
        ]
    );

    map.insert("Destructible".to_string(),
        vec![
            FieldDefinition2 {
            required:false, 
            iden: LitByteStr::new(DEST_CODE, Span::call_site()), 
            name: Ident::new(DEST_NAME, Span::call_site()), 
            field_type: FieldType::Custom(syn::parse_str(DEST_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(DSTD_CODE, Span::call_site()), 
                name: Ident::new(DSTD_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(DSTD_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(DSTF_CODE, Span::call_site()), 
                name: Ident::new(DSTF_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(DSTF_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(DMDL_CODE, Span::call_site()), 
                name: Ident::new(DMDL_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(DMDL_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(DMDT_CODE, Span::call_site()), 
                name: Ident::new(DMDT_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(DMDT_TYPE).unwrap())
            }
        ]
    );

    map.insert(PRPS_NAME.to_string(),
        vec![
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(PRPS_CODE, Span::call_site()), 
                name: Ident::new(PRPS_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(PRPS_TYPE).unwrap())
            }
        ]
    );

    map.insert("PickUpPutDown".to_string(),
        vec![
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(YNAM_CODE, Span::call_site()), 
                name: Ident::new(YNAM_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(YNAM_TYPE).unwrap())
            },
            FieldDefinition2 {
                required:false, 
                iden: LitByteStr::new(ZNAM_CODE, Span::call_site()), 
                name: Ident::new(ZNAM_NAME, Span::call_site()), 
                field_type: FieldType::Custom(syn::parse_str(ZNAM_TYPE).unwrap())
            }
        ]
    );

    map.insert(ATTX_NAME.to_string(),
        vec![
            FieldDefinition2 {
                required:false,
                iden: LitByteStr::new(ATTX_CODE, Span::call_site()),
                name: Ident::new(ATTX_NAME, Span::call_site()),
                field_type: FieldType::Custom(syn::parse_str(ATTX_TYPE).unwrap())
            }
        ]
    );

    map
}

// ====================================================================================================

pub struct FlagDefinition {
    pub position: LitInt,
    pub name: Ident
}


impl Parse for FlagDefinition {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let position = input.parse()?;
        input.parse::<Token![,]>()?;
        let name = input.parse()?;

        Ok(FlagDefinition { position, name })
    }
}



// ====================================================================================================

fn make_record_traits_impl(record_name: &Ident, record_field_name: &Ident, fields: &Punctuated<FieldDefinition2, Token![;]>) -> proc_macro2::TokenStream {
    
    let mut out = quote! {};

    for field in fields {
        let name = &field.name;

        match name.to_string().as_str() {
            EDID_NAME => {
                let field_name = Ident::new(EDID_NAME, name.span());
                out.extend(quote! {
                    fn try_get_editor_id(&self) -> Option<&ESMString> {
                        for field in &self.fields {
                            match field {
                                #record_field_name::#field_name(edid) => {
                                    return Some(edid);
                                },
                                _ => {}
                            }
                        }
                        None
                    }
                });
            }
            DESC_NAME => {
                let field_name = Ident::new(DESC_NAME, name.span());
                out.extend(quote! {
                    fn try_get_description(&self) -> Option<&LocalizedString> {
                        for field in &self.fields {
                            match field {
                                #record_field_name::#field_name(desc) => {
                                    return Some(desc);
                                },
                                _ => {}
                            }
                        }
                        None
                    }
                });
            }
            FULL_NAME => {
                let field_name = Ident::new(FULL_NAME, name.span());
                out.extend(quote! {
                    fn try_get_full_name(&self) -> Option<&LocalizedString> {
                        for field in &self.fields {
                            match field {
                                #record_field_name::#field_name(full) => {
                                    return Some(full);
                                },
                                _ => {}
                            }
                        }
                        None
                    }
                });
            }

            

            

            _ => { /* do nothing */ }
        }
    }


    quote! {
        impl RecordTraits for #record_name {
            #out
            fn get_record_header(&self) -> &RecordHeader {
                &self.header
            }
        }
    }
}