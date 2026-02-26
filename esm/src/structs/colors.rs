use crate::dev::*;


#[derive(Debug, NomLE, PartialEq)]
pub struct Color4 {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}


#[derive(Debug, NomLE, PartialEq)]
pub struct Color3 {
    pub r: u8,
    pub g: u8,
    pub b: u8
}


#[derive(Debug, NomLE, PartialEq)]
pub struct Color2 {
    pub r: u8,
    pub g: u8
}