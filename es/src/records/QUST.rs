use crate::{dev::*, groups::prelude::{CellVisibleDistantChildren, RawCellVisibleDistantChildren}};

define_record3! {
    "iden": b"QUST";
    "name": Quest;
    "child_type": CellVisibleDistantChildren;
    "fields": [
        EditorId;
        VirtualMachineAdapter;
        FullName;
        // TODO: A whole bunch of stuff
        // This record is not in the dump because it has sub groups
    ]
}

// ====================================================================================================

#[derive(Debug)]
pub enum RawQuestItem<'es> {
    Record(RawRecord<'es>),
    Children(RawCellVisibleDistantChildren<'es>)
}

// ====================================================================================================

impl<'es> Parse<&'es[u8]> for RawQuestItem<'es> {
    fn parse(i: &'es[u8]) -> IResult<&'es[u8], Self, nom::error::Error<&'es[u8]>> {
        
        let (_, next_id) = FourCC::parse(i)?;

        if &next_id.0 == b"GRUP" {

            let (i, children) = RawCellVisibleDistantChildren::parse(i)?;
            Ok((i, RawQuestItem::Children(children)))

        } else {

            let (i, record) = RawRecord::parse(i)?;
            Ok((i, RawQuestItem::Record(record)))
        }
    }
}

// ====================================================================================================

#[derive(Debug)]
pub struct RawQuestRecord<'es> {
    pub record: RawRecord<'es>,
    pub children: Option<RawCellVisibleDistantChildren<'es>>
}
impl RawQuestRecord<'_> {
    pub fn has_children(&self) -> bool {
        self.children.is_some()
    }
}

// impl <'es> Parse<&'es[u8]> for RawQuestRecord<'es>  {
//     fn parse(i: &'es[u8]) -> IResult<&'es[u8], Self> {
        
//         // TODO: NEED TO FIX, NON-ORIGIN groups can appear without their parent (yikes)

//         let (_, next_id) = FourCC::parse(i)?;

//         // Assume that the group is here by itself
//         if &next_id.0 == b"GRUP" {

//             let (i, children) = RawCellVisibleDistantChildren::parse(i)?;

//         } else {

//             let (i, record) = RawRecord::parse(i)?;

//         }


//         if !i.is_empty() {
//             println!("  Parsing after quest...");
//             let (_, gh) = GroupHeader::parse(i)?;

//             // Almost always this is the last record with no children
//             if let GroupLabel::Top(_) = gh.label {
//                 Ok((i, Self { record, children: None }))
//             } else if let GroupLabel::CellVisibleDistantChildren(_form_id) = gh.label {
//                 let (i, children) = RawCellVisibleDistantChildren::parse(i)?;

//                 Ok((i, Self { record, children: Some(children) }))


//             } else {
//                 if &gh.iden.0 != b"GRUP" {
//                     Ok((i, Self { record, children: None }))
//                 } else {
//                     panic!("Encountered unexpected group label after Quest record: {:?}", gh);
//                 }
//             }



//         } else {
//             println!("Data after quest is empty: {:?}", record.header);
//             Ok((i, Self { record, children: None }))
//         }
//     }
// }


// // ====================================================================================================