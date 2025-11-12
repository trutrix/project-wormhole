use crate::dev::*;


#[derive(PartialEq, Eq, Clone, PartialOrd, Ord, Hash, NomLE)]
pub struct FormId(pub u32);


// ================================================================================


impl std::fmt::Debug for FormId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FormId({:08X})({})", self.0, self.0)
    }
}


impl std::fmt::Display for FormId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:08X}", self.0)
    }
}


// ================================================================================