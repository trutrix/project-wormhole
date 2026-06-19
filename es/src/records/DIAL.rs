use crate::{dev::*, groups::prelude::{RawTopicChildren, TopicChildren}};


define_record3! {
    "iden": b"DIAL";
    "name": Dialog;
    "child_type": TopicChildren;
    "fields": [
        EditorId;
        FullName;
        b"PNAM", Priority, f32;
        b"BNAM", OwningBranch, FormId;
        b"QNAM", OwningQuest, FormId;
        b"DATA", Data, DialogData;
        b"SNAM", Subtype, FourCC;
        b"TIFC", InfoCount, u32;
    ]
}


// ====================================================================================================

#[derive(Debug, NomLE, PartialEq)]
pub struct DialogData {
    pub unknown1: u8,
    pub dialog_tab: u8,
    pub subtype_id: u8,
    pub unused: u8
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

        if !i.is_empty() {

            let (_, gh) = GroupHeader::parse(i)?;

            // Almost always this is the last record with no children
            if let GroupLabel::Top(_) = gh.label {
                Ok((i, Self { record, children: None }))
            } else if let GroupLabel::TopicChildren(_form_id) = gh.label {
                let (i, children) = RawTopicChildren::parse(i)?;

                Ok((i, Self { record, children: Some(children) }))


            } else {
                if &gh.iden.0 != b"GRUP" {
                    Ok((i, Self { record, children: None }))
                } else {
                    panic!("Encountered unexpected group label after Dialog record: {:?}", gh);
                }
            }



        } else {
            Ok((i, Self { record, children: None }))
        }

        // if record.header.iden.0 != *b"DIAL" {
        //     panic!("Encounterd non Dialog header: {:?}:", record.header);
        // }
        
        // if i.is_empty() {
        //     return Ok((i, Self { record, children: None }));
        // }

        // let (_, next_id) = FourCC::parse(i)?;

        // if next_id.0 != *b"GRUP" {
        //     return Ok((i, Self { record, children: None }));
        // }

        // let (_, next_header) = GroupHeader::parse(i)?;

        // match next_header.label {
        //     GroupLabel::TopicChildren(_) => {
        //         let (i, children) = RawTopicChildren::parse(i)?;
        //         Ok((i, Self { record, children: Some(children) }))
        //     }
        //     _ => {
        //         Ok((i, Self { record, children: None }))
        //     }
        // }

    }
}