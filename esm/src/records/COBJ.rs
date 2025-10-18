use crate::dev::*;

define_record! {
    b"COBJ",
    ConstructibleObject, [
        EditorId;
        Description;
        Condition;
        b"YNAM", PickupSound, FormId;
        b"ZNAM", PutDownSound, FormId;
        b"CNAM", CreatedObject, FormId;
        b"BNAM", WorkbenchKeyword, FormId;
        b"FVPA", Components, Vec<(FormId, u32)>;
        b"ANAM", MenuArtObject, FormId;
    ]
}