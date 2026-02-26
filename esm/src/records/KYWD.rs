use crate::{dev::*, structs::colors::Color4};


define_record2! {
    b"KYWD", Keyword, [
        EditorId;
        FullName;
        b"CNAM", Color, Color4;
        b"DNAM", Notes, ESMString;
        b"TNAM", Type, KeywordType;
        b"DATA", AttractionRule, u32;
        b"NNAM", DisplayName, ESMString;
    ]
}


#[derive(Debug, NomLE, Default, PartialEq)]
#[repr(u32)]
pub enum KeywordType {
    #[default]
    None = 0,
    ComponentTechLevel = 1,
    AttachPoint = 2,
    ComponentProperty = 3,
    InstantiationFilter = 4,
    ModAssociation = 5,
    Sound = 6,
    AnimArchetype = 7,
    FunctionCall = 8,
    RecipeFilter = 9,
    AttractionType = 10,
    DialogueSubtype = 11,
    QuestTarget = 12,
    AnimFlavor = 13,
    AnimGender = 14,
    AnimFace = 15,
    QuestGroup = 16,
    AnimInjured = 17,
    DispelEffect = 18,
}