use crate::dev::*;


#[derive(Debug)]
pub struct CellVisibleDistantChildren {
    pub header: GroupHeader,
    pub cells: Vec<u8>
}


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