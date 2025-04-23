use crate::dev::*;

define_record! {
    b"ADDN",
    Addon, [
        b"EDID", EditorId, ESMString;
    ]
}