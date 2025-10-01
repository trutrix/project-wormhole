use crate::dev::*;

// This record is usually compressed
// TODO: Figure out how to decompress within current limits
// For now, just parse the header and skip the rest


define_record! {
    b"NPC_",
    NonPlayerCharacter, [
    ]
}