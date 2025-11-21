use crate::dev::*;

define_record2! {
    b"SMBN",
    StoryManagerBranchNode, [
        EditorId;
        Condition;
        b"SNAM", PreviousSibling, FormId;
        b"PNAM", ParentNode, FormId;
        b"XNAM", MaxConcurrentQuests, u32;
        b"DNAM", Flags, StoryManagerBranchNodeFlags;
    ]
}

#[derive(Debug, NomLE)]
pub struct StoryManagerBranchNodeFlags {
    // TODO: bitflags
}