use crate::dev::*;


pub type ModelPath = ESMString;
pub type ModelColorMap = u8;
pub type ModelMaterialSwap = u8;
pub type ModelFlags = u32;



#[derive(Debug)]
pub struct ModelTexture {
    pub header_count: u8,
    pub texture_count: Option<u8>,
    pub addon_count: Option<u8>,
    pub texture_sets: Option<u8>,
    pub materials_count: Option<u8>,
    pub texture_path: ESMString
}



impl Parse<&[u8]> for ModelTexture {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header_count) = le_u8(i)?;

        let (i, texture_count) = if header_count >= 1 {
            let (i, v) = le_u8(i)?;
            (i, Some(v))
        } else {
            (i, None)
        };

        let (i, addon_count) = if header_count >= 2 {
            let (i, v) = le_u8(i)?;
            (i, Some(v))
        } else {
            (i, None)
        };

        let (i, texture_sets) = if header_count >= 3 {
            let (i, v) = le_u8(i)?;
            (i, Some(v))
        } else {
            (i, None)
        };

        let (i, materials_count) = if header_count >= 4 {
            let (i, v) = le_u8(i)?;
            (i, Some(v))
        } else {
            (i, None)
        };

        let (i, texture_path) = ESMString::parse(i)?;

        Ok((i, ModelTexture {
            header_count,
            texture_count,
            addon_count,
            texture_sets,
            materials_count,
            texture_path
        }))
    }
}