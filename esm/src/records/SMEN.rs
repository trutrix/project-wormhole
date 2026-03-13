use crate::dev::*;

define_record3! {
    "iden": b"SMEN";
    "name": StoryManagerEventNode;
    "fields": [
        EditorId;
        Condition;
        b"SNAM", PreviousSibling, FormId;
        b"PNAM", ParentNode, FormId;
        b"XNAM", MaxConcurrentQuests, u32;
        b"DNAM", Flags, StoryManagerEventNodeFlags;
        b"ENAM", Type, u32; // TODO: enum?
    ]
}

#[derive(Debug, NomLE)]
pub struct StoryManagerEventNodeFlags {
    // TODO: bitflags
}