use crate::{dev::*, records::all::Cell};

// ====================================================================================================

pub type ExteriorCellSubBlock = Group<Cell>;

// ====================================================================================================

pub type RawExteriorCellSubBlock<'esm> = Group<RawCellRecord<'esm>>;