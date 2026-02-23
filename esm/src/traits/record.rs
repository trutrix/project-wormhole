use crate::dev::*;



// These will be automatically implemented for most Records
pub trait RecordTraits {
    fn get_record_header(&self) -> &RecordHeader;
    fn get_form_id(&self) -> &FormId;
    fn try_get_editor_id(&self) -> Option<&ESMString> { None }
    fn try_get_full_name(&self) -> Option<&LocalizedString> { None }
    fn try_get_keywords(&self) -> Option<&Vec<FormId>> { None }
    fn try_get_description(&self) -> Option<&LocalizedString> { None }
    fn try_get_native_terminal(&self) -> Option<&FormId> { None }
    fn try_get_virtual_machine_adapter(&self) -> Option<&VirtualMachineAdapter> { None }
}


pub trait RecordTraits2 {
    fn get_record_header(&self) -> &RecordHeader;
    fn get_form_id(&self) -> &FormId;
}


pub trait EditorIdTrait {
    fn get_editor_id(&self) -> &EditorId;
}


pub trait FormIdTrait {
    fn get_form_id(&self) -> &FormId;
}