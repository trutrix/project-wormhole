use crate::dev::*;

define_record! {
    b"FLOR",
    Flora, [
        EditorId;
        FullName;
        AllModelData;
        VirtualMachineAdapter;
        Keywords;
        ObjectBounds;
        Destructible;
        PreviewTransform;
        b"PFPC", IngredientProduction, [u8;4];
        Properties;
        b"ATTX", ActivateTextOverride, LocalizedString;
        b"PNAM", Unknown1, u32; // TODO: figure out - always 4 bytes
        b"PFIG", Ingredient, FormId;
        b"SNAM", HarvestSound, FormId;
        b"FNAM", Unknown2, u16; // TODO: figure out - always 2 bytes
    ]
}