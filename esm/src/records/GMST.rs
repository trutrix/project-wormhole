use project_wormhole_proc::define_record3;

use crate::{dev::*, esm::ESMError, prelude::EditorIdTrait};

// define_record2! {
//     b"GMST",
//     GameSetting, [
//         EditorId;
//         b"DATA", Value, Vec<u8>;
//     ]
// }

define_record3! {
    "iden": b"GMST";
    "name": GameSetting;
    "fields": [
        EditorId;
        b"DATA", Value, Vec<u8>;
    ];
    "flags": [
        0x00000001, IsImportant;
    ];
    "fixed": true;
}

impl EditorIdTrait for GameSetting {
    fn get_editor_id(&self) -> &EditorId {
        &self.data.0
    }
}

impl GameSetting {
    fn get_value(&self) -> Result<EGameSettingValue, ESMError> {
        let edid = &self.data.0;
        let raw_value = &self.data.1;

        match edid.0.chars().nth(0) {
            Some(c) => {
                match c {
                    'b' => {
                        let value = raw_value[0] != 0;
                        return Ok(EGameSettingValue::Boolean(value));
                    }
                    'i' => {
                        let value = i32::from_le_bytes(raw_value[0..4].try_into().unwrap());
                        return Ok(EGameSettingValue::Integer(value));
                    }
                    'f' => {
                        let value = f32::from_le_bytes(raw_value[0..4].try_into().unwrap());
                        return Ok(EGameSettingValue::Float(value));
                    }
                    's' | 'S' => {
                        let value = String::from_utf8_lossy(&raw_value[0..]).to_string();
                        return Ok(EGameSettingValue::String(value));
                    }
                    'c' => {
                        let value = raw_value[0] as char;
                        return Ok(EGameSettingValue::Char(value));
                    }
                    'h' => {
                        let value = raw_value[0] as char;
                        return Ok(EGameSettingValue::HexChar(value));
                    }
                    'u' => {
                        let value = u32::from_le_bytes(raw_value[0..4].try_into().unwrap());
                        return Ok(EGameSettingValue::UnsignedInt(value));
                    }
                    'r' => {
                        let value = u32::from_le_bytes(raw_value[0..4].try_into().unwrap());
                        return Ok(EGameSettingValue::RGB(value));
                    }
                    'a' => {
                        let value = u32::from_le_bytes(raw_value[0..4].try_into().unwrap());
                        return Ok(EGameSettingValue::RGBA(value));
                    }
                    _ => {
                        return Ok(EGameSettingValue::Unknown(c as u8, raw_value.clone()));
                    }
                }
            }
            None => {
                return Err(ESMError::GameSetting("GameSetting::get_value() EditorId is empty".to_string()));
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
pub enum EGameSettingValue {
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