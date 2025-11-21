use crate::dev::*;

define_record2! {
    b"QUST",
    Quest, [
        EditorId;
        VirtualMachineAdapter;
        FullName;
        // TODO: A whole bunch of stuff
        // This record is not in the dump because it has sub groups
    ]
}