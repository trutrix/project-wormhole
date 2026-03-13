use crate::dev::*;

define_record3! {
    "iden": b"CLAS";
    "name": Class;
    "fields": [
        EditorId;
        FullName;
        Description;
        Properties;
        b"ICON", InventoryImage, u8; // TODO: unknown
        b"DATA", ClassData, (u32, f32); // Unknown / Bleedout?
    ]
}