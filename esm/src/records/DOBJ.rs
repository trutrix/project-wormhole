use crate::dev::*;


// Strange record, No EDID, just a big list

define_record3! {
    "iden": b"DOBJ";
    "name": DefaultObjects;
    "fields": [
        b"DNAM", Data, Vec<(ESMString, FormId)> // TODO: Editor Id / Ref?
    ]
}