use crate::dev::*;

pub trait RecordParser<T> where T: for<'esm> Parse<&'esm[u8]> {
    fn parse_body(i: &[u8]) -> IResult<&[u8], Vec<T>, nom::error::Error<&[u8]>> {
        let (i, fields) = many0(T::parse_le)(i)?;
        Ok((i, fields))
    }
    fn parse_record(i: &[u8]) -> IResult<&[u8], Record<T>, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;

        if header.flags.is_compressed() {
            if let Ok(dec) = decompress_record(raw) {
                if let Ok((_, fields)) = Self::parse_body(&dec) {
                    Ok((i, Record { header, fields }))
                } else {
                    println!("Failed to parse decompressed record: {:?}", header);
                    Ok((i, Record { header, fields: Vec::new() }))
                }
            } else {
                println!("Failed to decompress record: {:?}", header);
                Ok((i, Record { header, fields: Vec::new() }))
            }
        } else {
            let (_, fields) = Self::parse_body(raw)?;
            Ok((i, Record { header, fields }))
        }
    }
}




// These will be automatically implemented for most Records
pub trait RecordTraits {
    fn get_record_header(&self) -> &RecordHeader;
    fn get_form_id(&self) -> &FormId;
    fn try_get_editor_id(&self) -> Option<&ESMString> { None }
    fn try_get_full_name(&self) -> Option<&LocalizedString> { None }
    fn try_get_keywords(&self) -> Option<&Vec<FormId>> { None }
    fn try_get_description(&self) -> Option<&LocalizedString> { None }
    fn try_get_native_terminal(&self) -> Option<&FormId> { None }
    fn try_get_virtual_machine_adapter(&self) -> Option<&VirtualMachineAdapter> { None }
}
