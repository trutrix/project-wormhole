use crate::dev::{Field, Group, Record};

pub type FormId = u32;
pub type SkippedField = u8;



pub type TopGroup<T> = Group<Vec<Record<Vec<Field<T>>>>>;