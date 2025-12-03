use crate::{dev::*, records::all::NavMeshMapInfo};

#[derive(Debug)]
pub struct VirtualMachineAdapter {
    pub version: u16,
    pub object_format: u16,
    pub script_count: u16,
    pub scripts: Vec<VMADScriptEntry>
}


impl Parse<&[u8]> for VirtualMachineAdapter {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, version) = u16::parse_le(i)?;
        let (i, object_format) = u16::parse_le(i)?;
        let (i, script_count) = u16::parse_le(i)?;
        
        let mut loop_count = 0;
        let mut scripts = Vec::new();
        let mut i = i;

        while loop_count < script_count {
            let (i_new, script) = VMADScriptEntry::parse_versioned(i, version)?;
            scripts.push(script);
            loop_count += 1;
            i = i_new;
        }

        Ok((i, VirtualMachineAdapter { version, object_format, script_count, scripts }))
    }
}


#[derive(Debug)]
pub struct VMADScriptEntry {
    pub script_name: SizedString16,
    pub status: u8,
    pub property_count: u16,
    pub properties: Vec<VMADPropertyEntry>,
    pub fragments: Vec<u8>
}

impl VMADScriptEntry {
    pub fn parse_versioned<'esm>(i: &'esm[u8], version: u16) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, script_name) = SizedString16::parse(i)?;
        //println!("Parsed VMAD script name: {}", script_name);
        let (i, status) = u8::parse_le(i)?;
        let (i, property_count) = u16::parse_le(i)?;
        let (i, properties) = nom::multi::count(VMADPropertyEntry::parse, property_count as usize)(i)?;

        Ok((i, VMADScriptEntry { script_name, status, property_count, properties, fragments: Vec::new() }) )
    }
}


impl Parse<&[u8]> for VMADScriptEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, script_name) = SizedString16::parse(i)?;
        //println!("Parsed VMAD script name: {}", script_name);
        let (i, status) = u8::parse_le(i)?;
        let (i, property_count) = u16::parse_le(i)?;
        let (i, properties) = nom::multi::count(VMADPropertyEntry::parse, property_count as usize)(i)?;

        Ok((i, VMADScriptEntry { script_name, status, property_count, properties, fragments: Vec::new() }) )
    }
}

#[derive(Debug)]
pub struct VMADPropertyEntry {
    pub name: SizedString16,
    pub type_: u8,
    pub status: u8,
    pub value: VMADPropertyValue
}


impl Parse<&[u8]> for VMADPropertyEntry {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, name) = SizedString16::parse(i)?;
        let (i, type_) = u8::parse_le(i)?;
        let (i, status) = u8::parse_le(i)?;

        match type_ {
            1 => {
                // Object
                let (i, v1) = u16::parse_le(i)?;
                let (i, v2) = u16::parse_le(i)?;
                let (i, v3) = FormId::parse_le(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::Object(VMADObjectRef::V2((v1, v2, v3))) }))
            },
            2 => {
                // String
                let (i, value) = SizedString16::parse(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::String(value) }))
            },
            3 => {
                // Int
                let (i, value) = i32::parse_le(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::Int(value) }))
            },
            4 => {
                // Float
                let (i, value) = f32::parse_le(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::Float(value) }))
            },
            5 => {
                // Bool
                let (i, value) = u8::parse_le(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::Bool(value != 0) }))
            },
            11 => {
                // Object Array
                let (i, item_count) = u32::parse_le(i)?;
                let (i, values) = nom::multi::count(FormId::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::ObjectArray(values) }))
            },
            12 => {
                // String Array
                let (i, item_count) = u32::parse_le(i)?;
                let (i, values) = nom::multi::count(SizedString16::parse, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::StringArray(values) }))
            },
            13 => {
                // Int Array
                let (i, item_count) = u32::parse_le(i)?;
                let (i, values) = nom::multi::count(i32::parse_le, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::IntArray(values) }))
            },
            14 => {
                // Float Array
                let (i, item_count) = u32::parse_le(i)?;
                let (i, values) = nom::multi::count(f32::parse_le, item_count as usize)(i)?;
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::FloatArray(values) }))
            },
            15 => {
                // Bool Array
                let (i, item_count) = u32::parse_le(i)?;
                let (i, raw_values) = nom::multi::count(u8::parse_le, item_count as usize)(i)?;
                let values: Vec<bool> = raw_values.iter().map(|&b| b != 0).collect();
                Ok((i, VMADPropertyEntry { name, type_, status, value: VMADPropertyValue::BoolArray(values) }))
            },
            _ => {
                panic!("Unsupported VMAD property type: {}", type_);
            }
            
        }

    }
}


#[derive(Debug)]
pub enum VMADPropertyValue {
    Object(VMADObjectRef),
    String(SizedString16),
    Int(i32),
    Float(f32),
    Bool(bool),

    ObjectArray(Vec<FormId>),
    StringArray(Vec<SizedString16>),
    IntArray(Vec<i32>),
    FloatArray(Vec<f32>),
    BoolArray(Vec<bool>),
}

#[derive(Debug)]
pub enum VMADObjectRef {
    V1((FormId, u16, u16)),
    V2((u16, u16, FormId))
}