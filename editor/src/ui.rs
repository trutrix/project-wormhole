pub mod navbar;
pub mod style;
pub mod pages;
pub mod components;


pub mod all {
    pub use super::navbar::*;
    pub use super::style::*;
    pub use super::pages::all::*;
    pub use super::components::*;
}