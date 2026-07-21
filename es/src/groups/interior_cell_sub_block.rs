use crate::{dev::*, records::all::{Cell, RawCellRecord}};

// ====================================================================================================

pub type InteriorCellSubBlock = GroupOld<Cell>;

// ====================================================================================================

pub type RawInteriorCellSubBlock<'esm> = GroupOld<RawCellRecord<'esm>>;