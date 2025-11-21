use crate::dev::*;


// Strange record, No EDID, just a big list

define_record2! {
    b"DOBJ",
    DefaultObjects, [
        b"DNAM", Data, Vec<(ESMString, FormId)> // TODO: Editor Id / Ref?
    ]
}