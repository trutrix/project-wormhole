use crate::{dev::*, groups::cell_children::CellChildItem};



#[derive(Debug)]
pub struct CellPersistentChildren {
    pub header: GroupHeader,
    pub children: Vec<CellChildItem>,
}

impl Parse<&[u8]> for CellPersistentChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        // Parse header and raw data pointer
        let (i, (header, raw)) = alloc_group(i)?;

        // Ensure correct group type - debugging only
        #[cfg(debug_assertions)]
        match header.label {
            GroupLabel::CellPersistentChildren(_) => { }
            _ => { panic!("CellPersistentChildren::parse encountered wrong group type: {:?}", header.label) }
        }

        // Parse all child items from remaining raw data
        let (_, children) = many0(CellChildItem::parse)(raw)?;

        Ok((i, Self { header, children }) )
    }
}