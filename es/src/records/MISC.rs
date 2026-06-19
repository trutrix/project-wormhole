use crate::dev::*;

define_record3! {
    "iden": b"MISC";
    "name": MiscItem;
    "fields": [
        EditorId;
        ObjectBounds;
        PreviewTransform;
        VirtualMachineAdapter;
        ModelData;
        FullName;
        Keyword;
        Destructible;
        PickUpPutDown;
        b"DATA", Data, ValueWeight;
        b"CVPA", Components, Vec<(FormId, u32)>; // (Component, Count)
        b"FIMD", FeaturedItemMessage, LocalizedString; // TODO: confirm type
        b"CDIX", ComponentDisplayIndices, u32;
    ]
}