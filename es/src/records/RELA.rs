use crate::dev::*;

define_record3! {
    "iden": b"RELA";
    "name": Relationship;
    "fields": [
        EditorId;
        b"DATA", Data, RelationshipData;
    ]
}


// TODO: verify field types and sizes
// total struct size: 16 bytes
#[derive(Debug, NomLE, PartialEq, Eq)]
pub struct RelationshipData {
    pub parent: FormId,
    pub child: FormId,
    pub rank: u16,
    pub unknown1: u8,
    pub flags: u8,
    pub association_type: FormId,
}