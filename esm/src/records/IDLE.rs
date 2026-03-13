use crate::dev::*;

define_record3! {
    "iden": b"IDLE";
    "name": IdleAnimation;
    "fields": [
        EditorId;
        Condition;
        b"DATA", Data, IdleData;
        b"ENAM", AnimEvent, ESMString; // TODO: verify type
        b"GNAM", AnimFile, ESMString;
        b"DNAM", BehaviorGraph, ESMString;
        b"ANAM", RelatedAnims, (FormId, FormId);
    ]
}

// 6 bytes
#[derive(Debug, NomLE, PartialEq)]
pub struct IdleData {

}