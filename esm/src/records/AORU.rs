use crate::dev::*;

define_record! {
    b"AORU",
    AttractionRule, [
        EditorId;
        b"AOR2", AttractionRuleData, AttractionRuleData;
    ]
}


#[derive(Debug, NomLE)]
pub struct AttractionRuleData {

}