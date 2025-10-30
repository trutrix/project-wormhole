use crate::dev::*;


#[derive(Debug, NomLE, Clone, PartialEq)]
pub struct ValueWeight {
    pub value: u32,
    pub weight: f32
}