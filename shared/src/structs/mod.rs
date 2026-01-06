pub mod color;
pub mod u8_bool;
pub mod math;
pub mod sized_string;
pub mod fourcc;


pub mod prelude {
    pub use crate::structs::fourcc::*;
    pub use crate::structs::sized_string::*;
    pub use crate::structs::math::*;
}