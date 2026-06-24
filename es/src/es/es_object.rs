use crate::{dev::*, es::{es_group::ESGroup, es_record::ESRecord}};


#[derive(Debug, Readable, Writable)]
pub enum ESObject {
    Record(ESRecord),
    Group(ESGroup)
}