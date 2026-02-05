use crate::dev::*;


// TODO: Verify data, just because it parses doesn't mean it's correct!
#[derive(Debug)]
pub struct VirtualMachineAdapter {
    pub version: i16,
    pub object_format: i16,
    pub script_count: u16,
    pub scripts: Vec<VMADScriptEntry>
}


impl Parse<&[u8]> for VirtualMachineAdapter {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, version) = le_i16(i)?;

        #[cfg(debug_assertions)]
        {
            if version < 4 {
                println!("Parsing VMAD with version {}", version);
            }
        }


        let (i, object_format) = le_i16(i)?;
        let (i, script_count) = le_u16(i)?;
        
        let mut loop_count = 0;
        let mut scripts = Vec::new();
        let mut remaining = i;
        let mut depth = 0;

        // for debugging
        // if script_count > 0 {
        //     println!("VMAD Header: version: {}, format: {}, script_count: {}", version, object_format, script_count);
        // }

        depth += 1;

        while loop_count < script_count {
            let (i_new, script) = VMADScriptEntry::parse_versioned(remaining, version)?;
            scripts.push(script);
            loop_count += 1;
            remaining = i_new;
        }

        Ok((remaining, VirtualMachineAdapter { version, object_format, script_count, scripts }))
    }
}


#[derive(Debug)]
pub struct VMADScriptEntry {
    pub script_name: SizedString16,
    pub flags: VMADScriptFlags,
    pub property_count: u16,
    pub properties: Vec<VMADPropertyEntry>,
    pub fragments: Vec<u8>
}

impl<'nom> ParseVersioned<'nom, i16, nom::error::Error<&'nom[u8]>> for VMADScriptEntry {
    fn parse_versioned(i: &[u8], version: i16) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, script_name) = SizedString16::parse(i)?;
        //println!("Parsed VMAD script name: {}", script_name);
        let (i, flags) = if version >= 4 {
            VMADScriptFlags::parse(i)?
        } else {
            (i, VMADScriptFlags::empty())
        };
        let (i, property_count) = le_u16(i)?;
        if property_count == 0 {
            return Ok((i, VMADScriptEntry { script_name, flags, property_count, properties: Vec::new(), fragments: Vec::new() }) )
        }
        let (i, properties) = nom::multi::count(VMADPropertyEntry::parse, property_count as usize)(i)?;

        Ok((i, VMADScriptEntry { script_name, flags, property_count, properties, fragments: Vec::new() }) )
    }
}


// impl Parse<&[u8]> for VMADScriptEntry {
//     fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
//         let (i, script_name) = SizedString16::parse(i)?;
//         //println!("Parsed VMAD script name: {}", script_name);
//         let (i, status) = le_u8(i)?;
//         let (i, property_count) = le_u16(i)?;
//         let (i, properties) = nom::multi::count(VMADPropertyEntry::parse, property_count as usize)(i)?;

//         Ok((i, VMADScriptEntry { script_name, status, property_count, properties, fragments: Vec::new() }) )
//     }
// }

#[derive(Debug)]
pub struct VMADPropertyEntry {
    pub name: SizedString16,
    pub type_: VMADPropertyType,
    pub flags: u8,
    pub value: VMADPropertyValue
}


impl Parse<&[u8]> for VMADPropertyEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, name) = SizedString16::parse(i)?;
        let (i, type_) = VMADPropertyType::parse(i)?;
        let (i, flags) = le_u8(i)?;

        match type_ {
            VMADPropertyType::Null => {
                // Null
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Null }))
            }
            VMADPropertyType::Object => {
                // Object
                let (i, v1) = le_u16(i)?;
                let (i, v2) = le_u16(i)?;
                let (i, v3) = FormId::parse(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Object(VMADObjectRef::V2((v1, v2, v3))) }))
            },
            VMADPropertyType::String => {
                // String
                let (i, value) = SizedString16::parse(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::String(value) }))
            },
            VMADPropertyType::Int => {
                // Int
                let (i, value) = le_i32(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Int(value) }))
            },
            VMADPropertyType::Float => {
                // Float
                let (i, value) = le_f32(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Float(value) }))
            },
            VMADPropertyType::Bool => {
                // Bool
                let (i, value) = le_u8(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Bool(value != 0) }))
            },
            // 6 => {
            //     // Null
            //     Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Null }))
            // },
            VMADPropertyType::Struct => {
                // Unsupported
                let (i, value) = VMADPropertyEntry::parse(i)?;
                println!("{:?}", value);
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Struct(Box::new(value)) }))
            }
            VMADPropertyType::ObjectArray => {
                // Object Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(FormId::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::ObjectArray(values) }))
            },
            VMADPropertyType::StringArray => {
                // String Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(SizedString16::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::StringArray(values) }))
            },
            VMADPropertyType::IntArray => {
                // Int Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(le_i32, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::IntArray(values) }))
            },
            VMADPropertyType::FloatArray => {
                // Float Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(le_f32, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::FloatArray(values) }))
            },
            VMADPropertyType::BoolArray => {
                // Bool Array
                let (i, item_count) = le_u32(i)?;
                let (i, raw_values) = nom::multi::count(le_u8, item_count as usize)(i)?;
                let values: Vec<bool> = raw_values.iter().map(|&b| b != 0).collect();
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::BoolArray(values) }))
            },
            VMADPropertyType::VarArray => {
                // Var Array
                panic!("VarArray parsing not implemented yet!");
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::VarArray}))
            },
            VMADPropertyType::StructArray => {
                // Struct Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(SubStruct::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::StructArray(values)}))
            }
            _ => {

                #[cfg(debug_assertions)]
                {
                    println!("Encountered unsupported VMAD property type: {:?} for {}", type_, name);
                }


                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Unsupported}))
            }
            
        }

    }
}

impl<'esm> ParseVersioned<'esm, i16, nom::error::Error<&'esm[u8]>> for VMADPropertyEntry {
    fn parse_versioned(i: &[u8], version: i16) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, name) = SizedString16::parse(i)?;
        let (i, type_) = VMADPropertyType::parse(i)?;
        let (i, flags) = le_u8(i)?;

        println!("VMAD property name: {}, type: {:?}, flags: {}", name, type_, flags);

        match type_ {
            VMADPropertyType::Null => {
                // Null
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Null }))
            }
            VMADPropertyType::Object => {
                // Object
                let (i, v1) = le_u16(i)?;
                let (i, v2) = le_u16(i)?;
                let (i, v3) = FormId::parse_le(i)?;
                

                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Object(VMADObjectRef::V2((v1, v2, v3))) }))
            },
            VMADPropertyType::String => {
                // String
                let (i, value) = SizedString16::parse(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::String(value) }))
            },
            VMADPropertyType::Int => {
                // Int
                let (i, value) = le_i32(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Int(value) }))
            },
            VMADPropertyType::Float => {
                // Float
                let (i, value) = le_f32(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Float(value) }))
            },
            VMADPropertyType::Bool => {
                // Bool
                let (i, value) = le_u8(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Bool(value != 0) }))
            },
            // 6 => {
            //     // Null
            //     Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Null }))
            // },
            VMADPropertyType::Struct => {
                // Unsupported
                let (i, value) = VMADPropertyEntry::parse(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Struct(Box::new(value)) }))
            }
            VMADPropertyType::ObjectArray => {
                // Object Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(FormId::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::ObjectArray(values) }))
            },
            VMADPropertyType::StringArray => {
                // String Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(SizedString16::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::StringArray(values) }))
            },
            VMADPropertyType::IntArray => {
                // Int Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(le_i32, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::IntArray(values) }))
            },
            VMADPropertyType::FloatArray => {
                // Float Array
                let (i, item_count) = le_u32(i)?;
                let (i, values) = nom::multi::count(le_f32, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::FloatArray(values) }))
            },
            VMADPropertyType::BoolArray => {
                // Bool Array
                let (i, item_count) = le_u32(i)?;
                let (i, raw_values) = nom::multi::count(le_u8, item_count as usize)(i)?;
                let values: Vec<bool> = raw_values.iter().map(|&b| b != 0).collect();
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::BoolArray(values) }))
            },
            VMADPropertyType::VarArray => {
                // Var Array
                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::VarArray}))
            },
            VMADPropertyType::StructArray => {
                // Struct Array
                let (i, item_count) = le_u32(i)?;

                // let (i, sub_count) = le_u32(i)?;
                // println!("{}VMAD property struct array sub-count {}", depth_to_space(depth), sub_count);

                // let (i, st) = SizedString16::parse(i)?;
                // println!("{}VMAD property struct array type name: {}", depth_to_space(depth), st);

                // todo!()
                if let Ok((i, values)) = nom::multi::count(SubStruct::parse, item_count as usize)(i) {
                    Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::StructArray(values)}))    
                } else {
                    panic!("Failed to parse struct array items!");
                    Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::StructArray(Vec::new())}))
                }
            }
            _ => {

                #[cfg(debug_assertions)]
                {
                    println!("Encountered unsupported VMAD property type: {:?} for {}", type_, name);
                }


                Ok((i, VMADPropertyEntry { name, type_, flags, value: VMADPropertyValue::Unsupported}))
            }
        }
    }
}


// ====================================================================================================

#[derive(Debug)]
pub enum VMADPropertyValue {
    Null, // 0, 6
    Object(VMADObjectRef), // 1
    String(SizedString16), // 2
    Int(i32), // 3
    Float(f32), // 4
    Bool(bool), // 5
    Struct(Box<VMADPropertyEntry>), // 7
    ObjectArray(Vec<FormId>), // 11
    StringArray(Vec<SizedString16>), // 12
    IntArray(Vec<i32>), // 13
    FloatArray(Vec<f32>), // 14
    BoolArray(Vec<bool>), // 15
    VarArray, // 16
    StructArray(Vec<SubStruct>), // 17
    Unsupported
}

// ====================================================================================================

#[derive(Debug, NomLE)]
#[repr(u8)]
pub enum VMADPropertyType {
    // Singles
    Null = 0,
    Object = 1,
    String = 2,
    Int = 3,
    Float = 4,
    Bool = 5,
    Struct = 7,
    Var = 8,
    // Lists
    ObjectArray = 11,
    StringArray = 12,
    IntArray = 13,
    FloatArray = 14,
    BoolArray = 15,
    VarArray = 16,
    StructArray = 17,
    Unsupported
}

// ====================================================================================================

#[derive(Debug)]
pub enum VMADObjectRef {
    V1((FormId, u16, u16)),
    V2((u16, u16, FormId))
}


pub fn depth_to_space(depth: u8) -> String {
    let mut s = String::new();
    for _ in 0..depth {
        s.push_str("  ");
    }
    s
}


bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct VMADScriptFlags: u8 {
        const NONE = 0x00;
        const EDITED = 0x01;
        const UNKNOWN1 = 0x02;
        const REMOVED = 0x04;
    }
}

impl Parse<&[u8]> for VMADScriptFlags {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, raw) = le_u8(i)?;
        Ok((i, VMADScriptFlags::from_bits_truncate(raw)))
    }
}


// ====================================================================================================

#[derive(Debug, NomLE)]
pub struct SubStruct {
    #[nom(LengthCount = "le_u32")]
    pub values: Vec<VMADPropertyEntry>
}