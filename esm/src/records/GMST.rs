use proc::define_record;

use crate::{dev::*, esm::ESMError, traits::{GroupParser, RecordParser}};


define_record! {
    b"GMST",
    GameSetting, [
        b"EDID", EditorId, ESMString;
        b"DATA", Value, Vec<u8>;
    ]
}

impl RecordParser<GameSettingField> for GameSetting {}
impl GroupParser<GameSetting> for Group<Record<GameSettingField>> {}

// impl GameSetting {
//     fn get_value(&self) -> Result<GameSettingValue, ESMError> {
//         let edid = match &self.fields.get(0) {
//             Some(GameSettingField::EditorId(edid)) => edid,
//             _ => return Err(ESMError::GameSetting("GameSetting::get_value() Invalid EditorId".to_string())),
//         };

//         let raw_value = match &self.fields.get(1) {
//             Some(GameSettingField::Value(value)) => value,
//             _ => return Err(ESMError::GameSetting("GameSetting::get_value() Invalid Value".to_string())),
//         };

//         match edid.0.chars().nth(0) {
//             Some(c) => {
//                 match c {
//                     'b' => {
//                         let value = raw_value[0] != 0;
//                         return Ok(GameSettingValue::Boolean(value));
//                     }
//                     'i' => {
//                         let value = i32::from_le_bytes(raw_value[0..4].try_into().unwrap());
//                         return Ok(GameSettingValue::Integer(value));
//                     }
//                     'f' => {
//                         let value = f32::from_le_bytes(raw_value[0..4].try_into().unwrap());
//                         return Ok(GameSettingValue::Float(value));
//                     }
//                     's' | 'S' => {
//                         let value = String::from_utf8_lossy(&raw_value[0..]).to_string();
//                         return Ok(GameSettingValue::String(value));
//                     }
//                     'c' => {
//                         let value = raw_value[0] as char;
//                         return Ok(GameSettingValue::Char(value));
//                     }
//                     'h' => {
//                         let value = raw_value[0] as char;
//                         return Ok(GameSettingValue::HexChar(value));
//                     }
//                     'u' => {
//                         let value = u32::from_le_bytes(raw_value[0..4].try_into().unwrap());
//                         return Ok(GameSettingValue::UnsignedInt(value));
//                     }
//                     'r' => {
//                         let value = u32::from_le_bytes(raw_value[0..4].try_into().unwrap());
//                         return Ok(GameSettingValue::RGB(value));
//                     }
//                     'a' => {
//                         let value = u32::from_le_bytes(raw_value[0..4].try_into().unwrap());
//                         return Ok(GameSettingValue::RGBA(value));
//                     }
//                     _ => {
//                         return Ok(GameSettingValue::Unknown(c as u8, raw_value.clone()));
//                     }
//                 }
//             }
//             None => {
//                 return Err(ESMError::GameSetting("GameSetting::get_value() EditorId is empty".to_string()));
//             }
//         }
//     }
// }

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