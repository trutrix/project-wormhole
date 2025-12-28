use crate::dev::*;


#[derive(Debug, NomLE)]
pub struct Color4 {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}


#[derive(Debug, NomLE)]
pub struct Color3 {
    r: u8,
    g: u8,
    b: u8
}


#[derive(Debug, NomLE)]
pub struct Color2 {
    r: u8,
    g: u8
}