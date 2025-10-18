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
        b"INTV", CreatedObjectCount, u16; // Sometimes 4 bytes
        b"NAM2", Nam2, FormId; // Always 4 bytes - Assuming FormId
        b"NAM1", Nam1, FormId; // Always 4 bytes - Assuming FormId
        b"FNAM", Categories, Vec<FormId>;
        
    ]
}