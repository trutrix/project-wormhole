use crate::{dev::*, records::all::{RawDialog}};

// ====================================================================================================


/// Unknown / unimplemented group
#[derive(Debug)]
pub struct CellVisibleDistantChildren {
    pub header: GroupHeader,
    pub cells: Vec<u8>
}

// ====================================================================================================

impl Parse<&[u8]> for CellVisibleDistantChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::CellVisibleDistantChildren(_) => {
                let cells = raw.to_vec();
                Ok((i, Self { header, cells }) )
            }
            _ => { panic!("CellVisibleDistantChildren::parse encountered wrong group type: {:?}", header.label) }
        }

    }
}


// ====================================================================================================


#[derive(Debug)]
pub struct RawCellVisibleDistantChildren<'esm> {
    pub header: GroupHeader,
    pub branches: Vec<RawCellVisibleDistantChild<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawCellVisibleDistantChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;

        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::CellVisibleDistantChildren(_) => { }
            _ => {
                panic!("Unexpected group encountered. {:?}", header);
            }
        }

        let (_, branches) = many0(RawCellVisibleDistantChild::parse)(data)?;

        Ok((i, Self { header, branches }))
    }
}

// ====================================================================================================

#[derive(Debug)]
pub enum RawCellVisibleDistantChild<'esm> {
    Dialog(RawDialog<'esm>),
    DialogBranch(RawRecord<'esm>),
    Scene(RawRecord<'esm>)
}


impl<'esm> Parse<&'esm[u8]> for RawCellVisibleDistantChild<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (_, next_id) = FourCC::parse(i)?;

        match &next_id.0 {
            b"DIAL" => {
                let (i, record) = RawDialog::parse(i)?;
                Ok((i, RawCellVisibleDistantChild::Dialog(record)))
            }
            b"DLBR" => {
                let (i, record) = RawRecord::parse(i)?;
                Ok((i, RawCellVisibleDistantChild::DialogBranch(record)))
            }
            b"SCEN" => {
                let (i, record) = RawRecord::parse(i)?;
                Ok((i, RawCellVisibleDistantChild::Scene(record)))
            }
            _ => {
                if next_id.0 == *b"GRUP" {
                    let (_, header) = GroupHeader::parse(i)?;
                    panic!("Wrong RawCellVisibleDistantChild encountered: {:?}" , header);
                } else {
                    let (_, header) = RecordHeader::parse(i)?;
                    panic!("Wrong RawCellVisibleDistantChild encountered: {:?}" , header);
                }
                
            }
        }

    }
}