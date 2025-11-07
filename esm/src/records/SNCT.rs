use crate::dev::*;

define_record! {
    b"SNCT",
    SoundCategory, [
        EditorId;
        FullName;
        b"PNAM", ParentCategory, FormId;
        b"ONAM", MenuSliderCategory, FormId;
        b"VNAM", StaticVolumeMultiplier, u16; // TODO: maybe a half?
        b"CNAM", SideChainMultiplier, f32;
        b"MNAM", MinFrequencyMultiplier, f32;
        b"FNAM", Flags, u32; // TODO: bitflags
        b"UNAM", DefaultMenuValue, u16; // TODO: maybe a half?
    ]
}