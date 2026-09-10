use crate::strings::*;

#[derive(Debug, Default, PartialEq)]
pub enum ESGame {
    ES4,
    ES5,
    FNV,
    FO3,
    #[default]
    FO4,
    SFD
}


impl ESGame {
    pub fn try_from_dir(path: &std::path::PathBuf) -> Option<ESGame> {
        if !path.is_dir() {
            // Do not panic, this is just trying to guess
            return None;
        } else {
            match path.file_name() {
                Some(s) => {
                    match s.to_str().unwrap_or("None") {
                        S_ES4_TITLE => Some(ESGame::ES4), 
                        S_ES5_TITLE => Some(ESGame::ES5),
                        S_FNV_TITLE => Some(ESGame::FNV), 
                        S_FO3_TITLE => Some(ESGame::FO3),
                        S_FO4_TITLE => Some(ESGame::FO4),
                        S_SFD_TITLE => Some(ESGame::SFD),
                        _ => None
                    }
                }
                None => None
            }
        }

        
    }

    pub fn get_full_title(&self) -> &str {
        match self {
            ESGame::ES4 => S_ES4_TITLE,
            ESGame::ES5 => S_ES5_TITLE,
            ESGame::FNV => S_FNV_TITLE,
            ESGame::FO3 => S_FO3_TITLE,
            ESGame::FO4 => S_FO4_TITLE,
            ESGame::SFD => S_SFD_TITLE
        }
    }

    pub fn get_short_title(&self) -> &str {
        match self {
            ESGame::ES4 => S_ES4_SHORT_TITLE,
            ESGame::ES5 => S_ES5_SHORT_TITLE,
            ESGame::FNV => S_FNV_SHORT_TITLE,
            ESGame::FO3 => S_FO3_SHORT_TITLE,
            ESGame::FO4 => S_FO4_SHORT_TITLE,
            ESGame::SFD => S_SFD_SHORT_TITLE
        }
    }
}