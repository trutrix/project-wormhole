use crate::{dev::*, esm::ESMError, traits::FieldParser};

use super::{field::FieldHeader, record::{RawRecord, RecordHeader}};

#[derive(Debug)]
pub struct FileHeader {
    pub header: RecordHeader,
}

impl TryFrom<RawRecord<'_>> for FileHeader {
    type Error = ESMError;
    fn try_from(value: RawRecord<'_>) -> Result<Self, Self::Error> {
        Ok(Self { header: value.header })
    }
}


#[derive(Debug)]
pub enum FileHeaderField {
    InteriorCellCount(u32),
    Author(String),
    Unknown(FourCC, usize)
}


impl FieldParser<FileHeaderField> for FileHeaderField {
    fn parse_field(i: &[u8]) -> IResult<&[u8], (super::field::FieldHeader, FileHeaderField), nom::error::Error<&[u8]>> {
        let (i, header) = FieldHeader::parse(i)?;
        let (i, raw) = take(header.size())(i)?;

        match &header.iden().0 {
            b"INCC" => {
                let (i, count) = le_u32(raw)?;
                Ok((i, (header, FileHeaderField::InteriorCellCount(count))))
            }
            _=> {
                Ok((i, (header, FileHeaderField::Unknown(*header.iden(), raw.len()))))
            }
        }
    }
}