

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct FourCC(pub [u8; 4]);


// ================================================================================


impl std::fmt::Debug for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.0).unwrap_or("????");
        write!(f, "FourCC({})", s)
    }
}

impl std::fmt::Display for FourCC {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.0).unwrap_or("????");
        write!(f, "{}", s)
    }
}

impl Default for FourCC {
    fn default() -> Self {
        FourCC(*b"DFLT")
    }
}


// ================================================================================


impl serde::Serialize for FourCC {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        serializer.serialize_str(String::from_utf8(self.0.to_vec()).expect("Could not convert bytes into utf8 string").as_str())
    }
}


impl<'de> serde::Deserialize<'de> for FourCC {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
 
        struct FourCCVisitor;

        impl<'de> serde::de::Visitor<'de> for FourCCVisitor {
            type Value = FourCC;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a FourCC code as 4 bytes")
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
                where
                    E: serde::de::Error, {
                if v.len() != 4 {
                    return Err(E::custom(format!("Expected 4 bytes for FourCC, got {}", v.len())));
                }
                let mut arr = [0u8; 4];
                arr.copy_from_slice(v);
                Ok(FourCC(arr))
            }
        }

        deserializer.deserialize_bytes(FourCCVisitor)
    }
}


// ================================================================================

impl nom_derive::Parse<&[u8]> for FourCC {
    fn parse(i: &[u8]) -> nom_derive::nom::IResult<&[u8], Self, nom_derive::nom::error::Error<&[u8]>> {
        let (i, code) = <[u8; 4]>::parse(i)?;
        Ok((i, FourCC(code)))
    }
}