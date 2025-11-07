use crate::dev::*;

define_record! {
    b"SMQN",
    StoryManagerQuestNode, [
        EditorId;
        Condition;
        b"PNAM", ParentNode, FormId;
        b"SNAM", PreviousSibling, FormId;
        b"XNAM", MaxConcurrentQuests, u32;
        b"MNAM", QuestRunCount, u32;
        b"HNAM", HoursToReset, f32;
        b"DNAM", Flags, [u16; 2]; // TODO: bitflags
        b"RNAM", QuestHoursToReset, f32;
        b"NNAM", Quest, FormId;
        b"QNAM", QuestCount, u32;
    ]
}