use crate::dev::*;

// ====================================================================================================

#[derive(PartialEq, Eq, Clone, Copy, PartialOrd, Ord, Hash, NomLE, Readable, Writable)]
pub struct FormId(pub u32);

// ====================================================================================================

impl std::fmt::Debug for FormId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "FormId(NULL)")
        } else {
            write!(f, "FormId({:08X})({})", self.0, self.0)
        }
        
    }
}

// ====================================================================================================


impl std::fmt::Display for FormId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_null() {
            write!(f, "FormId(NULL)")
        } else {
            write!(f, "{:08X}", self.0)
        }
    }
}


// ====================================================================================================

impl FormId {
    pub fn is_null(&self) -> bool {
        self.0 == 0
    }
}

// ====================================================================================================

impl From<[u8;4]> for FormId {
    fn from(value: [u8;4]) -> Self {
        Self(u32::from_le_bytes(value))
    }
}