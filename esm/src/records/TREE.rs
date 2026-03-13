use crate::dev::*;

// Appears only once in Fallout4.esm, maybe not used in Fallout 4


define_record3! {
    "iden": b"TREE";
    "name": Tree;
    "fields": [
        EditorId;
        VirtualMachineAdapter;
        FullName;
        ObjectBounds;
        ModelData;
        b"CNAM", Data, TreeData;
        b"PFPC", IngredientProductionFactor, [u8;4];
    ]
}


// size: 48
#[derive(Debug, NomLE)]
pub struct TreeData {
    pub values: [f32; 12]
}