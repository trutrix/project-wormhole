use crate::dev::*;

define_record3! {
    "iden": b"IPDS";
    "name": ImpactDataSet;
    "fields": [
        EditorId;
        b"PNAM", MaterialImpact, (FormId, FormId); // Material / Impact Data pairings
    ]
}