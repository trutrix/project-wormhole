use crate::{dev::*, records::all::RawDialogBranch};

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
    pub branches: Vec<RawDialogBranch<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawCellVisibleDistantChildren<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;


        let (_, branches) = many0(RawDialogBranch::parse)(data)?;

        Ok((i, Self { header, branches }))
    }
}
