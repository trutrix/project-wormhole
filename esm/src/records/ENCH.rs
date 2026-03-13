use crate::dev::*;


// Also known as Enchantment

define_record3! {
    "iden": b"ENCH";
    "name": ObjectEffect;
    "fields": [
        EditorId;
        ObjectBounds;
        FullName;
        Condition;
        b"EFID", BaseEffect, FormId;
        b"EFIT", Modifier, ObjectEffectModifier;
        b"ENIT", Data, ObjectEffectData;
    ]
}


// size 12 bytes
#[derive(Debug, NomLE)]
pub struct ObjectEffectModifier {
    pub magnitude: f32,
    pub area: u32,
    pub duration: u32,
}

// size 36 bytes
#[derive(Debug, NomLE)]
pub struct ObjectEffectData {
    pub enchantment_cost: u32,
    pub flags: u32,
    pub cast_type: u32,
    pub enchantment_amount: u32,
    pub target_type: u32,
    pub enchantment_type: u32,
    pub charge_time: f32,
    pub base_enchantment: FormId,
    pub worn_rescrict: FormId
}