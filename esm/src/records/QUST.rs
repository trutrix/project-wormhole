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
pub struct RawQuestRecord<'esm> {
    pub quest: RawRecord<'esm>,
    pub quest_children: Option<RawCellVisibleDistantChildren<'esm>>
}
impl RawQuestRecord<'_> {
    pub fn has_children(&self) -> bool {
        self.quest_children.is_some()
    }
}

impl <'esm> Parse<&'esm[u8]> for RawQuestRecord<'esm>  {
    fn parse(i: &'esm[u8]) -> IResult<&'esm[u8], Self> {


        // Parse the quest record first
        let (i, quest) = RawRecord::parse(i)?;

        println!("  Parsed {:?}", quest.header);

        
        // If the next thing isn't a group, return immediately
        let (_, next_id) = FourCC::parse(i)?;
        if &next_id.0 != b"GRUP" {
            return Ok((i, Self { quest, quest_children: None }));
        }

        // Treat next as a group and check if it belongs to this quest record
        let (_, ghead) = GroupHeader::parse(i)?;
        match ghead.label {
            GroupLabel::CellVisibleDistantChildren(_) => {
                println!("  Parsing CellVisibleDistantChildren...");
                let (i, quest_children) = RawCellVisibleDistantChildren::parse(i)?;
                Ok((i, Self { quest, quest_children: Some(quest_children) }))
            }
            _ => {
                //panic!("Wrong group after quest: {:?}", ghead);
                Ok((i, Self { quest, quest_children: None }))
            }
        }
    }
}


// ====================================================================================================

#[derive(Debug)]
pub enum RawQuestChild {
    Dialog,
    DialogBranch,
}