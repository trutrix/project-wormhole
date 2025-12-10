

pub const CELL_WIDTH_UNIT: f32 = 4096.0;
pub const CELL_WIDTH_M: f32 = 58.5;
pub const CELL_WIDTH_CM: f32 = CELL_WIDTH_M * 100.0;
pub const CELL_WIDTH_MM: f32 = CELL_WIDTH_M * 1000.0;
pub const UE_TO_CE_SCALE: f32 = 1.428_222_7; // Unvalidated

pub const GRUP: &[u8; 4] = b"GRUP";


// ====================================================================================================

pub const REFERENCE_GROUPS: [&[u8;4];3] = [b"WRLD", b"CELL", b"QUST"];