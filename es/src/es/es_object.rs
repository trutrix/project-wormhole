use crate::{dev::*, es::{es_group::ESGroup, es_record::ESRecord}};

// ===================================================================================================

#[derive(Debug)]
pub enum ESObject {
    Group(ESGroup),
    Record(ESRecord),
}

// ===================================================================================================

impl<'a, C: speedy::Context> speedy::Readable<'a, C> for ESObject {
    #[inline]
    fn read_from<R: speedy::Reader<'a, C>>(reader: &mut R) -> std::result::Result<Self, C::Error> {
        let kind = reader.peek_u32()?;

        match kind {
            // GRUP
            1196578128 => {
                let t0: ESGroup = reader.read_value()?;
                Ok(ESObject::Group(t0))
            }
            _ => {
                let t0: ESRecord = reader.read_value()?;
                Ok(ESObject::Record(t0))
            }
            _ => Err(speedy::private::error_invalid_enum_variant()),
        }
    }
}