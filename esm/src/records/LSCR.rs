use crate::dev::*;

define_record! {
    b"LSCR",
    LoadingScreen, [
        EditorId;
        Condition;
        Description;
        b"NNAM", LoadingScreenNIF, FormId; // SCOL
        b"TNAM", Transform, FormId; // TRNS
        b"ONAM", Rotation, (i16, i16); // MIN MAX
        b"ZNAM", Zoom, (f32, f32); // MIN MAX
    ]
}