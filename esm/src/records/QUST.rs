use crate::dev::*;

define_record3! {
    "iden": b"QUST";
    "name": Quest;
    "fields": [
        EditorId;
        VirtualMachineAdapter;
        FullName;
        // TODO: A whole bunch of stuff
        // This record is not in the dump because it has sub groups
    ]
}