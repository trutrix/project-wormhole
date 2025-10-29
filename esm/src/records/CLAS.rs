use crate::dev::*;

define_record! {
    b"CLAS",
    Class, [
        EditorId;
        FullName;
        Description;
        b"ICON", InventoryImage, u8; // TODO: unknown
        Properties;
        b"DATA", ClassData, (u32, f32); // Unknown / Bleedout?
    ]
}