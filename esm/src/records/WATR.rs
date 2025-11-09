use crate::dev::*;

define_record! {
    b"WATR",
    Water, [
        EditorId;
        FullName;
        b"DNAM", VisualData, EmptyParser; // TODO: large struct

        b"NAM0", LinearVelocity, [f32;3];
        b"NAM1", AngularVelocity, [f32;3];
        b"NAM2", Layer1NoiseTexture, ESMString;
        b"NAM3", Layer2NoiseTexture, ESMString;
        b"NAM4", Layer3NoiseTexture, ESMString;

        b"TNAM", Material, FormId;
        b"SNAM", Sound, FormId;

        b"XNAM", ConsumeSpell, FormId;
        b"YNAM", ContactSpell, FormId;
        b"INAM", ImageSpace, FormId;

        b"FNAM", Flags, u8; // TODO: bitfields
        b"ANAM", Opacity, u8;
        b"DATA", Data, EmptyParser; // TODO: present but always zero
        b"GNAM", Unknown1, [u32;3]; // TODO: verify type - always 12 bytes

    ]
}