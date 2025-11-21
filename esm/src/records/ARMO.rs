use crate::dev::*;

define_record2! {
    b"ARMO",
    Armor, [
        EditorId;
        ObjectBounds;
        ModelData;
        PreviewTransform;
        Description;

        b"KSIZ", KeywordCount, u32;
        b"KWDA", Keywords, Vec<FormId>; // Refs

        b"MOD2", ModelPath2, ModelPath;
        b"MOD3", ModelPath3, ModelPath;
        b"MOD4", ModelPath4, ModelPath;
        b"MOD5", ModelPath5, ModelPath;

        b"MO2T", ModelTexture2, ModelTexture;
        b"MO3T", ModelTexture3, ModelTexture;
        b"MO4T", ModelTexture4, ModelTexture;
        b"MO5T", ModelTexture5, ModelTexture;

        b"MO2F", ModelFlags2, ModelFlags;
        b"MO3F", ModelFlags3, ModelFlags;
        b"MO4F", ModelFlags4, ModelFlags;
        b"MO5F", ModelFlags5, ModelFlags;

        b"MO2S", ModelMaterialSwap2, ModelMaterialSwap;
        b"MO3S", ModelMaterialSwap3, ModelMaterialSwap;
        b"MO4S", ModelMaterialSwap4, ModelMaterialSwap;
        b"MO5S", ModelMaterialSwap5, ModelMaterialSwap;

        VirtualMachineAdapter;
        PickUpPutDown;

        b"DATA", Data, u8; // Unknown struct
        b"FNAM", Data2, u8; // Unknown struct

        b"DAMA", DamageResistances, u8; // Unknown struct
        b"TNAM", TemplateArmor, FormId; // Ref
        b"INRD", InstancedNaming, u8; // Unknown

        b"EITM", ObjectEffect, u8; // Unknown
        b"INDX", Unknown2, u8; // Unknown
        
        b"STOP", Unknown3, u8; // Unknown
        b"OBTS", ObjectTemplate, u8; // Unknown
        b"APPR", AttachParentSlots, u8; // Unknown
        b"BAMT", AlternateBlockMaterial, u8; // Unknown
        b"OBTE", ObjectTemplateExtra, u8; // Unknown
        b"OBTF", ObjectTemplateFlags, u8; // Unknown

        FullName;
        b"ETYP", EquipmentType, u8; // Unknown struct
        b"BOD2", BipedBodyTemplate, u8; // Unknown struct
        b"RNAM", Race, FormId; // Ref
    ]
}

// Field dump - {ETYP, EITM, APPR, MO2S, ZNAM, BAMT, OBTS, MO2T, OBTF, RNAM, MO4S, FNAM, DATA, FULL, BOD2, INDX, VMAD, MOD2, MODL, KWDA, OBTE, KSIZ, OBND, MOD4, PTRN, DAMA, DESC, YNAM, STOP, MO4T, INRD}