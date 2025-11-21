use crate::dev::*;

define_record2! {
    b"BPTD",
    BodyPartData, [
        EditorId;
        ModelData;

        // Compound data list
        b"BPTN", BodyPartName, LocalizedString;
        b"BPNN", BodyPartNode, ESMString;
        b"BPNT", BodyPartVats, ESMString;
        b"BPND", BodyPartDestruction, u8; // TODO: large struct
        b"NAM1", LimbReplacementModel, ESMString;
        b"NAM4", GoreTargetBone, ESMString;
        b"NAM5", ModelInfo, u8; // TODO: unknown
        b"ENAM", HitReactionStart, ESMString;
        b"FNAM", HitReactionEnd, ESMString;
        b"DNAM", TwistVariablePrefix, ESMString;
    ]
}