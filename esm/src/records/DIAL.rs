use crate::{dev::*, groups::prelude::RawTopicChildren};


define_record3! {
    "iden": b"DIAL";
    "name": Dialog;
    "fields": [
        EditorId;
    ]
}


// ====================================================================================================

#[derive(Debug)]
pub struct RawDialog<'esm> {
    pub record: RawRecord<'esm>,
    pub children: Option<RawTopicChildren<'esm>>
}

impl<'esm> Parse<&'esm[u8]> for RawDialog<'esm> {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self, nom::error::Error<&'esm[u8]>> {
        let (i, record) = RawRecord::parse(i)?;

        if record.header.iden.0 != *b"DIAL" {
            panic!("Encounterd non DialogBranch {:?}:", record.header);
        }
        
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
                Ok((i, Self { record, children: None }))
            }
        }

    }
}