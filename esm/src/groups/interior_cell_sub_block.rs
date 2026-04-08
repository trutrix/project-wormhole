use crate::{dev::*, records::all::Cell};

// ====================================================================================================

pub type InteriorCellSubBlock = Group<Cell>;

// ====================================================================================================

pub type RawInteriorCellSubBlock<'esm> = Group<RawCellRecord<'esm>>;