use crate::dev::*;

define_record3! {
    "iden": b"ARMA";
    "name": ArmorAddon;
    "fields": [
        EditorId;
        ModelData;

        b"BOD2", BipedBodyTemplate, u8; // Unknown struct
        b"RNAM", Race, FormId; // Ref
        b"DNAM", Data, u8; // Unknown struct

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

        b"NAM0", MaleSkinTexture, u8; // Unknown
        b"NAM1", FemaleSkinTexture, u8; // Unknown
        b"NAM2", MaleSkinTextureSwapList, u8; // Unknown
        b"NAM3", FemaleSkinTextureSwapList, u8; // Unknown

        b"SNDD", SoundData, FormId; // Ref
        b"ONAM", ArtObject, FormId; // Ref

        b"BSMP", Unknown1, u8; // Unknown
        b"BSMS", Unknown2, u8; // Unknown
        b"BSMB", Unknown3, u8; // Unknown

    ]
}

// Field dump - {BSMB, MO3C, NAM0, MO5S, MO3S, MODL, MOD3, BOD2, MO3F, NAM2, MO2F, ONAM, RNAM, MO3T, BSMS, MOD4, SNDD, MO2S, MO4T, BSMP, MO4S, MO5T, MOD2, MOD5, NAM1, MO2T}