use crate::dev::*;

define_record! {
    b"COBJ",
    ConstructibleObject, [
        EditorId;
        b"DESC", Description, ESMString;
        b"YNAM", PickupSound, FormId;
        b"ZNAM", PutDownSound, FormId;
        b"CTDA", Condition, u8;
        b"CIS1", ConditionParam1, u8;
        b"CIS2", ConditionParam2, u8;
        b"CNAM", CreatedObject, FormId;
        b"BNAM", WorkbenchKeyword, FormId;
        b"FVPA", Components, Vec<(FormId, u32)>;
        b"ANAM", MenuArtObject, FormId;
    ]
}