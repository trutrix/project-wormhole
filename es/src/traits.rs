use crate::dev::*;

// ====================================================================================================

pub mod record;
pub mod group;
pub mod field;
pub mod chunk;

// ====================================================================================================

pub mod prelude {
    pub use super::record::*;
    pub use super::group::*;
    pub use super::field::*;
}

// ====================================================================================================

pub trait ValidateData {
    fn is_valid(&self) -> bool;
}

// ====================================================================================================

// Parsing that does not expect a return buffer
pub trait ParseAllocated<H, R> where Self: Sized {
    fn parse_allocated(header: H, raw: R) -> Result<Self, nom::error::Error<R>>;
}

// This simplifies functions but makes a bunch of empty buffers
pub trait ParseAllocated2<H, R> where Self: Sized {
    fn parse_allocated2(header: H, raw: R) -> IResult<R, Self>;
}