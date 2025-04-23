use crate::dev::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, NomLE, PartialOrd, Ord)]
pub struct FourCC(pub [u8;4]);


impl std::fmt::Debug for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl std::fmt::Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}

impl PartialEq<&[u8;4]> for FourCC {
    fn eq(&self, other: &&[u8;4]) -> bool {
        self.0 == **other
    }
}