use crate::dev::*;

define_record3! {
    "iden": b"MGEF";
    "name": MagicEffect;
    "fields": [
        EditorId;
        Keyword;
        FullName;
        VirtualMachineAdapter;
        Condition;
        b"DATA", Data, MagicEffectData;
        b"DNAM", ItemDescription, LocalizedString;
        b"SNDD", Sounds, EmptyParser; //(FormId, FormId); // TODO: Sometimes field is zero length to denote NULL
    ]
}


#[derive(Debug, NomLE, PartialEq)]
pub struct MagicEffectData {
    // TODO: fill out
}