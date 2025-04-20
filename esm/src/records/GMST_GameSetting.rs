use crate::dev::*;

#[derive(Debug)]
pub struct GameSetting {
    pub header: RecordHeader,
    pub edid: String,
    pub data: GameSettingValue,
}

impl Parse<&[u8]> for GameSetting {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, header) = RecordHeader::parse(i)?;

        let (i, edid) = String::parse(i)?;
        let (i, data_header) = FieldHeader::parse(i)?;
        match edid.chars().next() {
            Some(c) => {
                match c {
                    'b' => {                  
                        let (i, data) = le_u32(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::Boolean(data != 0) }))
                    }
                    'i' => {
                        let (i, data) = le_i32(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::Integer(data) }))
                    }
                    'f' => {
                        let (i, data) = le_f32(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::Float(data) }))
                    }
                    's' | 'S' => {
                        let (i, data) = String::parse(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::String(data) }))
                    }
                    'c' => {
                        let (i, data) = le_u8(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::Char(data as char) }))
                    }
                    'h' => {
                        let (i, data) = le_u8(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::HexChar(data as char) }))
                    }
                    'u' => {
                        let (i, data) = le_u32(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::UnsignedInt(data) }))
                    }
                    'r' => {
                        let (i, data) = le_u32(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::RGB(data) }))
                    }
                    'a' => {
                        let (i, data) = le_u32(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::RGBA(data) }))
                    }
                    _ => {
                        let (i, data) = take(data_header.size())(i)?;
                        Ok((i, GameSetting { header, edid, data: GameSettingValue::Unknown(c as u8, data.to_vec()) }))
                    }
                }
            }
            None => {
                Ok((i, GameSetting { header, edid: String::new(), data: GameSettingValue::Unknown(0, Vec::new()) }))
            }
        }
    }
}

// b	boolean	uint32	0	Boolean
// i	int	uint32	3	Integer value
// f	float	float32	5	Float value
// s or S	string	lstring	6	Localized string
// c	char	char	1	Character
// h	char	char	2	Hex character?
// u	unsigned int	uint32	4	Unsigned integer value
// r	RGB	uint32	7	RGB value (alpha channel set to 0xFF)
// a	RGBA	uint32	8	RGBA value
// 9	Any value that's not one of the above.

#[derive(Debug)]
pub enum GameSettingValue {
    Boolean(bool),
    Integer(i32),
    Float(f32),
    String(String),
    Char(char),
    HexChar(char),
    UnsignedInt(u32),
    RGB(u32),
    RGBA(u32),
    Unknown(u8, Vec<u8>), // Unknown type and data
}