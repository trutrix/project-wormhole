use crate::es::es_group::{ESGroupHeader, ESGroupLabel};

#[derive(Debug)]
pub enum ESTop {
    Unhandled(ESGroupHeader)
}


impl ESTop {
    fn parse_allocated(i: &[u8], header: ESGroupHeader) -> ESTop {
        match header.get_label() {
            ESGroupLabel::Top(iden) => {
                match &iden.0 {
                    _ => {
                        println!("{:?}", iden);
                        ESTop::Unhandled(header) 
                    }
                }
            }
            _ => {
                panic!("ESTopGroup::parse_with_header(): Encountered wrong group type \n{:?}", header)
            }
        }
    }
}