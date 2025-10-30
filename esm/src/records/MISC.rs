use crate::dev::*;

define_record! {
    b"MISC",
    MiscItem, [
        EditorId;
        ObjectBounds;
        PreviewTransform;
        VirtualMachineAdapter;
        AllModelData;
        FullName;
        Keywords;
        Destructible;
        PickUpPutDown;
        b"DATA", Data, ValueWeight;
        b"CVPA", Components, Vec<(FormId, u32)>; // (Component, Count)
        b"FIMD", FeaturedItemMessage, LocalizedString; // TODO: confirm type
        b"CDIX", ComponentDisplayIndices, u32;
    ]
}