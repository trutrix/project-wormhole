#![allow(dead_code)]
use core::panic;
use std::collections::HashMap;

use proc_macro2::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::parse::discouraged::Speculative;
use syn::{*, parse::Parse, punctuated::Punctuated};

use super::record_consts::*;

// -- TODO --
// Unify logic for FieldDefinition types (make a function)

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
        let record_iden_string = Ident::new(String::from_utf8_lossy(&record_iden.value()).to_string().as_str(), record_iden.span());
        let name = &self.name;
        let name_field = Ident::new(format!("{}Field", name.clone().to_string().as_str()).as_str(), name.span());
        let name_group = Ident::new(format!("{}Group", name.clone().to_string().as_str()).as_str(), name.span());


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
        // let trait_impl = make_record_traits_impl(name, &name_field, fields);

        tokens.extend(quote! {

            pub type #name = Record<Vec<#name_field>>;
            pub type #name_group = Group<#name>;

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

            impl From<#name> for crate::records::SingleRecord {
                fn from(value: #name) -> Self {
                    Self::#record_iden_string(value)
                }
            }
        });

        
    }
}


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

#[derive(Clone)]
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
                    impl crate::prelude::EditorIdTrait for #record_name {
                        fn get_editor_id(&self) -> &EditorId {
                            for field in self.fields {
                                match field {
                                    #record_field_name::#field_name(edid) => {
                                        return &edid;
                                    },
                                    _ => { /* skip */ }
                                }
                            }
                            panic!("EditorId field not found.")
                        }
                    }
                });
            }
            // DESC_NAME => {
            //     let field_name = Ident::new(DESC_NAME, name.span());
            //     out.extend(quote! {
            //         fn try_get_description(&self) -> Option<&LocalizedString> {
            //             for field in &self.fields {
            //                 match field {
            //                     #record_field_name::#field_name(desc) => {
            //                         return Some(desc);
            //                     },
            //                     _ => {}
            //                 }
            //             }
            //             None
            //         }
            //     });
            // }
            // FULL_NAME => {
            //     let field_name = Ident::new(FULL_NAME, name.span());
            //     out.extend(quote! {
            //         fn try_get_full_name(&self) -> Option<&LocalizedString> {
            //             for field in &self.fields {
            //                 match field {
            //                     #record_field_name::#field_name(full) => {
            //                         return Some(full);
            //                     },
            //                     _ => {}
            //                 }
            //             }
            //             None
            //         }
            //     });
            // }

            

            

            _ => { /* do nothing */ }
        }
    }


    // out = quote! {
    //     impl crate::prelude::RecordTraits for #record_name {
    //         #out
    //         fn get_record_header(&self) -> &RecordHeader {
    //             &self.header
    //         }

    //         fn get_form_id(&self) -> &FormId {
    //             &self.header.form_id
    //         }
    //     }
    // };

    quote! {
        #out

        
    }
}


// ====================================================================================================

pub struct RecordDefinition3 {
    pub iden: LitByteStr,
    pub name: Ident,
    pub flags: Option<Punctuated<FlagDefinition, Token![;]>>,
    pub fields: Punctuated<FieldDefinition2, Token![;]>,
    pub fixed: bool,
    pub child_type: Option<Type>
}

impl Parse for RecordDefinition3 {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let field_defs = input.parse_terminated(NamedDef::parse, Token![;])?;

        let iden = field_defs.iter().find_map(|f| {
            if let NamedDef::Iden(iden) = f {
                Some(iden.clone())
            } else {
                None
            }
        }).expect("iden is required.");

        let name = field_defs.iter().find_map(|f| {
            if let NamedDef::Name(name) = f {
                Some(name.clone())
            } else {
                None
            }
        }).expect("name is required.");

        let flags = field_defs.iter().find_map(|f| {
            if let NamedDef::Flags(flags) = f {
                Some(flags.clone())
            } else {
                None
            }
        });

        let fields = field_defs.iter().find_map(|f| {
            if let NamedDef::Fields(fields) = f {
                Some(fields.clone())
            } else {
                None
            }
        }).expect("fields are required.");

        let fixed = field_defs.iter().any(|f| {
            if let NamedDef::Fixed(v) = f {
                *v
            } else {
                false
            }
        });

        let child_type = field_defs.iter().find_map(|f| {
            if let NamedDef::ChildType(t) = f {
                Some(t.clone())
            } else {
                None
            }
        });

        Ok(RecordDefinition3 { iden, name, flags, fields, fixed, child_type })

    }
}

impl ToTokens for RecordDefinition3 {
    fn to_tokens(&self, tokens: &mut TokenStream) {

        let iden = &self.iden;
        let iden_as_ident = iden_to_ident(iden);
        let name = &self.name;
        let name_field = Ident::new(format!("{}Field", name.clone().to_string().as_str()).as_str(), name.span());
        let name_group = Ident::new(format!("{}Group", name.clone().to_string().as_str()).as_str(), name.span());
        let is_fixed = self.fixed;
        let fields = &self.fields;
        let field_names: Vec<Ident> = fields.iter().map(|f| f.name.clone()).collect();
        let field_types: Vec<&FieldType> = fields.iter().map(|f| &f.field_type).collect();
        let field_idens: Vec<proc_macro2::TokenStream> = fields.iter().map(|f| {
            let iden = &f.iden;
            quote! { #iden }
        }).collect();


        let type_token = if is_fixed {

            let mut field_list: Vec<Type> = Vec::new();

            for field in fields {
                match &field.field_type {
                    FieldType::Common(ident) => {
                        let map = common_map2();
                        let lookup = ident.to_string();
                        let result = map.get(&lookup);
                        if let Some(real_value) = result {
                            for field2 in real_value {
                                match &field2.field_type {
                                    FieldType::Common(_) => todo!("This message should not be here"),
                                    FieldType::Custom(c) => {
                                        field_list.push(c.clone());
                                    },
                                    FieldType::Reference(_) => todo!("This message should not be here"),
                                }
                            }
                        }
                    },
                    FieldType::Custom(c) => {
                        field_list.push(c.clone());
                    },
                    FieldType::Reference(lit_byte_strs) => {
                        let len = lit_byte_strs.len();
                        let t = ident_to_type_manual(Ident::new("FormId", lit_byte_strs[0].span()));
                        for _ in 0..len {
                            field_list.push(t.clone());
                        }
                    },
                }
            }

            if let Some(child_type) = &self.child_type {
                quote! { (Record<#(#field_list),*> ,  Option<#child_type>) }
            } else {
                quote! { Record<(#(#field_list),*)> }
            }
        } else {

            if let Some(child_type) = &self.child_type {
                quote! { (Record<Vec<#name_field>>, Option<#child_type>) }
            } else {
                quote! { Record<Vec<#name_field>> }
            }
        };


        let out = quote! {
            pub type #name = #type_token;
            pub type #name_group = Group<#name>;

            impl From<#name> for crate::records::SingleRecord {
                fn from(value: #name) -> Self {
                    Self::#iden_as_ident(value)
                }
            }

            #[derive(Debug, PartialEq)]
            pub enum #name_field {
                Unhandled(FourCC),
                #(#field_names(#field_types)),*
            }

            impl project_wormhole_shared::prelude::FourCCTrait for &#name_field {
                fn fourcc(&self) -> FourCC {
                    match self {
                        #(
                            #name_field::#field_names(_) => FourCC(*#field_idens),
                        )*
                        #name_field::Unhandled(iden) => iden.clone(),
                    }
                }
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

        };
        
        
        tokens.extend(out);

    }
}



pub enum NamedDef {
    Iden(LitByteStr),
    Name(Ident),
    Fields(Punctuated<FieldDefinition2, Token![;]>),
    Flags(Punctuated<FlagDefinition, Token![;]>),
    ChildType(Type),
    Fixed(bool)
}

impl Parse for NamedDef {
    fn parse(input: parse::ParseStream) -> Result<Self> {
        let name: LitStr = input.parse()?;
        input.parse::<Token![:]>()?;
        let s = name.value();

        match s.as_str() {
            "iden" => { Ok(NamedDef::Iden(input.parse()?)) }
            "name" => { Ok(NamedDef::Name(input.parse()?)) }
            "fields" => {
                let inner;
                bracketed!(inner in input);
                Ok(NamedDef::Fields(inner.parse_terminated(FieldDefinition2::parse, Token![;])?))
            }
            "flags" => {
                let inner2;
                bracketed!(inner2 in input);
                Ok(NamedDef::Flags(inner2.parse_terminated(FlagDefinition::parse, Token![;])?))
            }
            "child_type" => {
                Ok(NamedDef::ChildType(input.parse()?))
            }
            "fixed" => {
                let fixed_token: LitBool = input.parse()?;
                if fixed_token.value {
                    Ok(NamedDef::Fixed(true))
                } else {
                    Ok(NamedDef::Fixed(false))
                }
            }

            _ => {
                panic!("Unhandled field value.")
            }

        }
    }
}



fn ident_to_type_manual(ident: Ident) -> Type {
    let path_segment = PathSegment {
        ident,
        arguments: PathArguments::None,
    };

    let path = Path {
        leading_colon: None,
        segments: std::iter::once(path_segment).collect(),
    };

    Type::Path(TypePath {
        qself: None,
        path,
    })
}

fn iden_to_ident(iden: &LitByteStr) -> Ident {
    let s = iden.value();
    Ident::new(String::from_utf8_lossy(&s).to_string().as_str(), iden.span())
}