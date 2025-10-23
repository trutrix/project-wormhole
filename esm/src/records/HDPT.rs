use crate::dev::*;

define_record! {
    b"HDPT",
    HeadPart, [
        EditorId;
        FullName;
        AllModelData;
        Condition;
        b"TNAM", TextureSet, FormId;
        b"RNAM", ValidRaces, FormId;
        b"DATA", Flags, u8;
        b"PNAM", Type, u32;
        b"NAM0", PartListType, u32;
        b"NAM1", PartlistPath, ESMString;
    ]
}