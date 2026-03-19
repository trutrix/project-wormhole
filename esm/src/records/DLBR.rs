use crate::{dev::*, groups::prelude::CellVisibleDistantChildren};


define_record3! {
    "iden": b"DLBR";
    "name": DialogBranch;
    "child_type": CellVisibleDistantChildren;
    "fields": [
        EditorId;
    ]
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawDialogBranch<'esm> {
    pub record: RawRecord<'esm>,
    pub children: Option<RawTopicChildren<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawDialogBranch<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, record) = RawRecord::parse(i)?;
        
        if i.is_empty() {
            return Ok((i, Self { record, children: None }));
        }

        let (_, next_id) = FourCC::parse(i)?;

        if next_id.0 != *b"GRUP" {
            return Ok((i, Self { record, children: None }));
        }

        let (_, next_header) = GroupHeader::parse(i)?;

        match next_header.label {
            GroupLabel::TopicChildren(cvdc) => {
                let (i, children) = RawTopicChildren::parse(i)?;
                Ok((i, Self { record, children: Some(children) }))
            }
            _ => {
                return Ok((i, Self { record, children: None }));
            }
        }

    }
}