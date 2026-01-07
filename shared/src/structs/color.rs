use std::fmt::Debug;

use nom_derive::{NomLE, Parse};
use nom_derive::nom;



#[derive(Clone, Copy, PartialEq, Eq, NomLE, Default)]
pub struct Color4<T>([T; 4]);

impl<T> Color4<T> {
    pub fn r(&self) -> &T {
        &self.0[0]
    }
    pub fn g(&self) -> &T {
        &self.0[1]
    }
    pub fn b(&self) -> &T {
        &self.0[2]
    }
    pub fn a(&self) -> &T {
        &self.0[3]
    }
}


impl<T> Debug for Color4<T>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color4(R: {:?}, G: {:?}, B: {:?}, A: {:?})", self.r(), self.g(), self.b(), self.a())
    }
}


// ================================================================================

#[derive(Clone, Copy, PartialEq, Eq, NomLE, Default)]
pub struct Color3<T>([T; 3]);

impl<T> Color3<T> {
    pub fn r(&self) -> &T {
        &self.0[0]
    }
    pub fn g(&self) -> &T {
        &self.0[1]
    }
    pub fn b(&self) -> &T {
        &self.0[2]
    }
}

impl<T> Debug for Color3<T>
where T: Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color3(R: {:?}, G: {:?}, B: {:?})", self.r(), self.g(), self.b())
    }
}