use crate::{dev::*, records::all::{Cell, RawCellRecord}};

// ====================================================================================================

pub type InteriorCellSubBlock = Group<Cell>;

// ====================================================================================================

pub type RawInteriorCellSubBlock<'es> = Group<RawCellRecord<'es>>;