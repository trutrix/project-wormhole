use crate::{dev::*, es::es_group::{ESGroupTrait, ESGroupHeader, alloc_group}, traits::ParseAllocated};

// ====================================================================================================

#[derive(Debug)]
pub struct ESWorldChildren {
    pub header: ESGroupHeader
}

// ====================================================================================================

impl ESGroupTrait for ESWorldChildren {
    fn group_label(&self) -> super::ESGroupLabel {
        self.header.get_label()
    }

    fn group_size(&self) -> &u32 {
        &self.header.size
    }
}

// ====================================================================================================


impl ParseAllocated<ESGroupHeader, &[u8]> for ESWorldChildren {
    fn parse_allocated(header: ESGroupHeader, raw: &[u8]) -> Result<Self, nom::error::Error<&[u8]>> {
        todo!()
    }
}

// ====================================================================================================

impl Parse<&[u8]> for ESWorldChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let result = ESWorldChildren::parse_allocated(header, raw).unwrap();
        Ok((i, result))
    }
}

