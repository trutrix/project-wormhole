use crate::dev::*;

pub type FormId = u32;
pub type SkippedField = u8;
pub type EditorId = ESMString;
pub type RecordProperty = (FormId, f32);

pub type TopGroup<T> = Group<Vec<Record<Vec<Field<T>>>>>;