use crate::{dev::*, es::{self, es_group::{ESGroupHeader, ESGroupTraits, ESGroupTyped}, es_record::{ESRecordFlags, ESRecordHeader, ESRecordTraits, ESRecordTyped, ESVersionControl}}};

// ====================================================================================================

pub trait ESObject {
    fn object_count(&self) -> &usize;
    fn object_size(&self) -> &u32;    
}


pub fn parse_es_object(i: &[u8]) -> IResult<&[u8], dyn ESObject> {
    let (i, iden) = FourCC::parse(i)?;
    let (i, size) = le_u32(i)?;

    if &iden.0 == b"GRUP" {
        let (i, label_value) = <[u8;4]>::parse(i)?;
        let (i, label_type) = le_u32(i)?;
        let (i, version_control) = ESVersionControl::parse(i)?;
        todo!()
        //Ok((i ))
    } else {
        let (i, flags) = ESRecordFlags::parse(i)?;
        let (i, form_id) = FormId::parse(i)?;
        let (i, version_control) = ESVersionControl::parse(i)?;
        todo!()
        //Ok((i, ))
    }
}