use crate::dev::*;

define_record! {
    b"CLAS",
    Class, [
        EditorId;
        FullName;
        Description;
        Properties;
        b"ICON", InventoryImage, u8; // TODO: unknown
        b"DATA", ClassData, (u32, f32); // Unknown / Bleedout?
    ]
}