use crate::{dev::*, records::all::{Cell, RawCellRecord}};

// ====================================================================================================

pub type ExteriorCellSubBlock = Group<Cell>;

// ====================================================================================================

pub type RawExteriorCellSubBlock<'es> = Group<RawCellRecord<'es>>;