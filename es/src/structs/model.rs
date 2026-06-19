use nom_derive::nom::multi::count;

use crate::dev::*;


pub type ModelPath = ESMString;
pub type ModelColorMap = u8;
pub type ModelMaterialSwap = u8;
pub type ModelFlags = u32;



#[derive(Debug, PartialEq)]
pub struct ModelTexture {
    pub item_types_count: u32,
    pub item_counts: Vec<u32>,
    pub textures: Vec<FileHashInfo>,
    pub addons: Vec<FileHashInfo>,
    pub materials: Vec<FileHashInfo>,
}


impl ModelTexture {
    pub fn get_texture_count(&self) -> Option<&u32> {
        self.item_counts.first()
    }

    pub fn get_addon_count(&self) -> Option<&u32> {
        self.item_counts.get(1)
    }

    pub fn get_texture_set_count(&self) -> Option<&u32> {
        self.item_counts.get(2)
    }

    pub fn get_material_count(&self) -> Option<&u32> {
        self.item_counts.get(3)
    }
}

#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct FileHashInfo {
    pub file_hash: u32,
    pub file_ext: FourCC,
    pub folder_hash: u32,
}

impl Parse<&[u8]> for ModelTexture {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, item_types_count) = le_u32(i)?;

        let (i, item_counts) = count(le_u32, item_types_count as usize)(i)?;
        let item_counts_len = item_counts.len();


        let (i, textures) = if item_counts_len >= 1 {
            count(FileHashInfo::parse, item_counts[0] as usize)(i)?
        } else {
            (i, vec![])
        };


        let (i, addons) = if item_counts_len >= 2 {
            count(FileHashInfo::parse, item_counts[1] as usize)(i)?
        } else {
            (i, vec![])
        };


        let (i, materials) = if item_counts_len >= 4 {
            count(FileHashInfo::parse, item_counts[3] as usize)(i)?
        } else {
            (i, vec![])
        };


        Ok((i, ModelTexture {
            item_types_count,
            item_counts,
            textures,
            addons,
            materials
        }))
    }
}