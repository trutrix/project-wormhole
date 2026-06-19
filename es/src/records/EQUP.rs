use crate::dev::*;

define_record3! {
    "iden": b"EQUP";
    "name": EquipType;
    "fields": [
        EditorId;
        b"PNAM", Parents, Vec<FormId>;
        b"DATA", Flags, u32;
        b"ANAM", ConditionActor, FormId;
    ]
}