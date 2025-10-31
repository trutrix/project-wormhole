use crate::dev::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BethesdaGame {
    SF1,
    F76,
    FO4,
    FO4V2,
    FO4V3,
    FO3,
    FNV,
    ES5,
    ES4,
    SSE,
    #[default]
    All,
}



impl std::fmt::Display for BethesdaGame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BethesdaGame::SF1 => write!(f, "Starfield"),
            BethesdaGame::F76 => write!(f, "Fallout 76"),
            BethesdaGame::FO4 => write!(f, "Fallout 4"),
            BethesdaGame::FO4V2 => write!(f, "Fallout 4 v2"),
            BethesdaGame::FO4V3 => write!(f, "Fallout 4 v3"),
            BethesdaGame::FO3 => write!(f, "Fallout 3"),
            BethesdaGame::FNV => write!(f, "Fallout New Vegas"),
            BethesdaGame::ES5 => write!(f, "Skyrim"),
            BethesdaGame::ES4 => write!(f, "Oblivion"),
            BethesdaGame::SSE => write!(f, "Skyrim Special Edition"),
            BethesdaGame::All => write!(f, "All Games"),
            
        }
    }
}

impl nom_derive::Parse<&[u8]> for BethesdaGame {
    fn parse(i: &[u8]) -> nom::IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, game_id) = nom::number::complete::le_u32(i)?;
        //info!("BethesdaGame: {}", game_id);
        match BethesdaGame::try_from(game_id) {
            Ok(game) => Ok((i, game)),
            Err(_) => Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Tag))),
        }
    }
}


impl TryFrom<u32> for BethesdaGame {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            172 => Ok(BethesdaGame::SF1),
            155 => Ok(BethesdaGame::F76),
            130 => Ok(BethesdaGame::FO4),
            132 => Ok(BethesdaGame::FO4V2),
            139 => Ok(BethesdaGame::FO4V3),
            34 => Ok(BethesdaGame::FO3),
            83 => Ok(BethesdaGame::ES5),
            100 => Ok(BethesdaGame::SSE),
            _ => Err(()),
        }
    }

}

impl From<BethesdaGame> for u32 {
    fn from(game: BethesdaGame) -> u32 {
        match game {
            BethesdaGame::SF1 => 172,
            BethesdaGame::F76 => 155,
            BethesdaGame::FO4 => 130,
            BethesdaGame::FO4V2 => 132,
            BethesdaGame::FO4V3 => 139,
            BethesdaGame::FO3 => 34,
            BethesdaGame::ES5 => 83,
            BethesdaGame::SSE => 100,
            _ => {
                panic!("Invalid BethesdaGame")
            }
        }
    }
}