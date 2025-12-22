use crate::dev::*;

// #[derive(Clone, Copy, PartialEq, Eq, Hash, NomLE, PartialOrd, Ord)]
// pub struct FourCC(pub [u8;4]);


// impl std::fmt::Debug for FourCC {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", String::from_utf8_lossy(&self.0))
//     }
// }

// impl std::fmt::Display for FourCC {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", String::from_utf8_lossy(&self.0))
//     }
// }

// impl PartialEq<&[u8;4]> for FourCC {
//     fn eq(&self, other: &&[u8;4]) -> bool {s
//         self.0 == **other
//     }
// }

// impl PartialEq<FourCC> for &[u8;4] {
//     fn eq(&self, other: &FourCC) -> bool {
//         **self == other.0
//     }
// }

// impl serde::Serialize for FourCC {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         let s = String::from_utf8_lossy(&self.0);
//         serializer.serialize_str(&s)
//     }
// }