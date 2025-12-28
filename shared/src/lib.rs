pub mod utils;
pub mod traits;
pub mod structs;
pub use glam;

pub mod prelude {
    pub use crate::traits::prelude::*;
    pub use crate::structs::prelude::*;
    pub use crate::utils::prelude::*;
}