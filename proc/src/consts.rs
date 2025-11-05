pub const EDID_CODE: &[u8;4] = b"EDID";
pub const EDID_NAME: &str = "EditorId";
pub const EDID_TYPE: &str = "EditorId";

// ====================================================================================================

pub const DESC_CODE: &[u8;4] = b"DESC";
pub const DESC_NAME: &str = "Description";
pub const DESC_TYPE: &str = "LocalizedString";

// ====================================================================================================

pub const FULL_CODE: &[u8;4] = b"FULL";
pub const FULL_NAME: &str = "FullName";
pub const FULL_TYPE: &str = "LocalizedString";

// ====================================================================================================

pub const CTDA_CODE: &[u8;4] = b"CTDA";
pub const CTDA_NAME: &str = "Condition";
pub const CTDA_TYPE: &str = "Condition";

pub const CIS1_CODE: &[u8;4] = b"CIS1";
pub const CIS1_NAME: &str = "ConditionParameter1";
pub const CIS1_TYPE: &str = "ConditionParameter";
    
pub const CIS2_CODE: &[u8;4] = b"CIS2";
pub const CIS2_NAME: &str = "ConditionParameter2";
pub const CIS2_TYPE: &str = "ConditionParameter";

// ====================================================================================================

pub const OBND_CODE: &[u8;4] = b"OBND";
pub const OBND_NAME: &str = "ObjectBounds";
pub const OBND_TYPE: &str = "ObjectBounds";

// ====================================================================================================

pub const PTRN_CODE: &[u8;4] = b"PTRN";
pub const PTRN_NAME: &str = "PreviewTransform";
pub const PTRN_TYPE: &str = "FormId";

// ====================================================================================================

pub const KYWD_CODE: &[u8;4] = b"KYWD";
pub const KYWD_NAME: &str = "Keyword";
pub const KYWD_TYPE: &str = "FormId";

pub const KSIZ_CODE: &[u8;4] = b"KSIZ";
pub const KSIZ_NAME: &str = "KeywordListSize";
pub const KSIZ_TYPE: &str = "u32";

// ====================================================================================================

pub const VMAD_CODE: &[u8;4] = b"VMAD";
pub const VMAD_NAME: &str = "VirtualMachineAdapter";
pub const VMAD_TYPE: &str = "VirtualMachineAdapter";

// ====================================================================================================

pub const MODL_CODE: &[u8;4] = b"MODL";
pub const MODL_NAME: &str = "ModelPath";
pub const MODL_TYPE: &str = "ESMString";

pub const MODT_CODE: &[u8;4] = b"MODT";
pub const MODT_NAME: &str = "ModelTexture";
pub const MODT_TYPE: &str = "ModelTexture";

pub const MODC_CODE: &[u8;4] = b"MODC";
pub const MODC_NAME: &str = "ModelColorMap";
pub const MODC_TYPE: &str = "ModelColorMap";

pub const MODS_CODE: &[u8;4] = b"MODS";
pub const MODS_NAME: &str = "ModelMaterialSwap";
pub const MODS_TYPE: &str = "ModelMaterialSwap";

pub const MODF_CODE: &[u8;4] = b"MODF";
pub const MODF_NAME: &str = "ModelFlags";
pub const MODF_TYPE: &str = "ModelFlags";

// ====================================================================================================

pub const DEST_CODE: &[u8;4] = b"DEST";
pub const DEST_NAME: &str = "DestructibleHeader";
pub const DEST_TYPE: &str = "DestructibleHeader";

pub const DSTD_CODE: &[u8;4] = b"DSTD";
pub const DSTD_NAME: &str = "DestructibleStage";
pub const DSTD_TYPE: &str = "DestructibleStage";

pub const DSTF_CODE: &[u8;4] = b"DSTF";
pub const DSTF_NAME: &str = "DestructibleEnd";
pub const DSTF_TYPE: &str = "EmptyParser";

pub const DMDL_CODE: &[u8;4] = b"DMDL";
pub const DMDL_NAME: &str = "DestructibleModelPath";
pub const DMDL_TYPE: &str = "ESMString";

pub const DMDT_CODE: &[u8;4] = b"DMDT";
pub const DMDT_NAME: &str = "DestructibleModelTexture";
pub const DMDT_TYPE: &str = "ModelTexture";

// ====================================================================================================

pub const PRPS_CODE: &[u8;4] = b"PRPS";
pub const PRPS_NAME: &str = "Properties";
pub const PRPS_TYPE: &str = "RecordProperty";

// ====================================================================================================

pub const YNAM_CODE: &[u8;4] = b"YNAM";
pub const YNAM_NAME: &str = "PickUpSound";
pub const YNAM_TYPE: &str = "FormId";

pub const ZNAM_CODE: &[u8;4] = b"ZNAM";
pub const ZNAM_NAME: &str = "PutDownSound";
pub const ZNAM_TYPE: &str = "FormId";