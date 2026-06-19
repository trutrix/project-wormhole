use crate::dev::*;

define_record3! {
    "iden": b"CMPO";
    "name": Component;
    "fields": [
        EditorId;
        ObjectBounds;
        FullName;
        b"CUSD", CraftSound, FormId;
        b"DATA", AutoCalcValue, u8; // TODO: find actual type
        b"MNAM", ScrapItem, FormId;
        b"GNAM", ModScrapScalar, FormId;
    ]
}