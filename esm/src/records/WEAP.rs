use crate::dev::*;

define_record3! {
    "iden": b"WEAP";
    "name": Weapon;
    "fields": [
        EditorId;
        Description;
        Destructible;
        FullName;
        Keyword;
        ModelData;
        ObjectBounds;
        PickUpPutDown;
        PreviewTransform;
        VirtualMachineAdapter;


        b"EITM", ObjectEffect, FormId;
        b"MASE", MeleeAttackSpeed, u32; // TODO: enum?
        b"EAMT", EnchantmentAmount, u16;
        b"CRDT", CriticalData, WeaponCriticalData;
        b"APPR", AttachParentSlots, Vec<FormId>;
        b"BAMT", AlternateBlockMaterial, FormId;
        
        b"ETYP", EquipmentType, FormId;
        b"INAM", ImpactDataSet, FormId;
        b"STOP", EndMarker, EmptyParser;
        b"WAMD", AimModel, FormId;
        b"INRD", InstanceNaming, FormId;
        b"DAMA", DamageType, (FormId, u32); // Ref // ammount (may have multiple)
        b"FNAM", FeedbackData, EmptyParser; // TODO: large struct
        b"BIDS", BlockImpactDataSet, FormId;
        b"WZMD", WeaponZoom, FormId;
        b"NNAM", EmbeddedMod, FormId;
        b"DNAM", Data, EmptyParser; // TODO: large struct (132 bytes)
        b"LNAM", NPCAddAmmoList, FormId;

        b"OBTS", ObjectTemplateItem, EmptyParser; // TODO: Variable size struct
        b"OBTF", ObjectTemplateStart, EmptyParser;
        b"OBTE", ObjectTemplateCount, u32;

        b"MO4T", FirstPersonTextures, ModelTexture;
        b"MOD4", FirstPersonModelPath, ESMString;
        b"MO4S", FirstPersonMaterialSwap, ModelMaterialSwap;
        b"MO4C", FirstPersonColorSwap, ModelColorMap;
        b"MO4F", FirstPersonModelFlags, ModelFlags;

    ]
}


#[derive(Debug, NomLE, PartialEq)]
pub struct WeaponCriticalData {
    pub damage_multiplier: f32,
    pub charge_bonus: f32,
    pub effect: FormId
}