use crate::{dev::*, records::all::{Cell, RawCellRecord}};

// ====================================================================================================

pub type ExteriorCellSubBlock = GroupOld<Cell>;

// ====================================================================================================

pub type RawExteriorCellSubBlock<'esm> = GroupOld<RawCellRecord<'esm>>;