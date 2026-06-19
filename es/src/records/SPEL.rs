use crate::dev::*;

define_record3! {
    "iden": b"SPEL";
    "name": Spell;
    "fields": [
        EditorId;
        ObjectBounds;
        FullName;
        Keyword;
        Description;
        Condition;
        b"ETYP", EquipmentType, FormId;
        b"SPIT", Data, SpellData;
        b"EFID", EffectId, FormId;
        b"EFIT", EffectData, EmptyParser; // TODO: probably a common struct
    ]
}



/// Size 36 bytes
#[derive(Debug, NomLE, PartialEq)]
pub struct SpellData {
    // TODO: fill in
}