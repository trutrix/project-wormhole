use crate::dev::*;

define_record2! {
    b"REGN",
    Region, [
        EditorId;
        b"RCLR", Color, Color4;
        b"WNAM", WorldSpace, FormId;
        b"RPLC", PointList, Vec<[f32;2]>;
        b"RDOT", DataObject, EmptyParser; // Appears to be zero-length, maybe a marker?
        b"RDAT", DataHeader, [u16;4]; // TODO: 4 values
        b"RDMO", DataMod, FormId;
        b"RDSA", Sounds, EmptyParser;
        b"ANAM", OcculsionAccuracy, f32;
        b"RPLI", EdgeFalloff, u32;
        b"RGDS", Unknown1, EmptyParser; // TODO: zero-length
        b"RDWT", WeatherTypes, EmptyParser; // TODO: variable length
    ]
}