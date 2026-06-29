use crate::es::es_group::{ESGroupHeader, ESGroupLabel};

#[derive(Debug)]
pub enum ESTopGroup {
    Unhandled(ESGroupHeader)
}


impl ESTopGroup {
    fn parse_with_header(i: &[u8], header: ESGroupHeader) -> ESTopGroup {
        match header.get_label() {
            ESGroupLabel::Top(iden) => {
                match &iden.0 {
                    _ => ESTopGroup::Unhandled(header)
                }
            }
            _ => {
                panic!("ESTopGroup::parse_with_header(): Encountered wrong group type \n{:?}", header)
            }
        }
    }
}