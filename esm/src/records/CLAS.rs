use crate::dev::*;

define_record! {
    b"CLAS",
    Class, [
        EditorId;
        FullName;
        Description;
        b"ICON", InventoryImage, u8; // TODO: unknown
        b"PRPS", Properties, Vec<(FormId, f32)>; // AVIF ref / value
        b"DATA", ClassData, (u32, f32); // Unknown / Bleedout?
    ]
}