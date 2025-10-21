use crate::dev::*;

define_record! {
    b"EQUP",
    EquipType, [
        EditorId;
        b"PNAM", Parents, Vec<FormId>;
        b"DATA", Flags, u32;
        b"ANAM", ConditionActor, FormId;
    ]
}