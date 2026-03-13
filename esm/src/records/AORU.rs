use crate::dev::*;

define_record3! {
    "iden": b"AORU";
    "name": AttractionRule;
    "fields": [
        EditorId;
        b"AOR2", AttractionRuleData, AttractionRuleData;
    ]
}


#[derive(Debug, NomLE)]
pub struct AttractionRuleData {

}