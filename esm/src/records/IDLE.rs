use crate::dev::*;

define_record2! {
    b"IDLE",
    IdleAnimation, [
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
#[derive(Debug, NomLE)]
pub struct IdleData {

}