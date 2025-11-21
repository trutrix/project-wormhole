use crate::dev::*;

define_record2! {
    b"IPDS",
    ImpactDataSet, [
        EditorId;
        b"PNAM", MaterialImpact, (FormId, FormId); // Material / Impact Data pairings
    ]
}