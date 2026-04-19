use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::settings::{Theme, ThemeColor};

pub(crate) enum RawTheme {
    SystemDependent(ThemeColor),
    Light(ThemeColor),
    Dim,
    Dark(ThemeColor),
    Unknown(i32),
}

impl Serialize for RawTheme {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let color = match self {
            RawTheme::SystemDependent(color) | RawTheme::Light(color) | RawTheme::Dark(color) => {
                color
            }
            _ => &ThemeColor::Default,
        };
        let color_code = match color {
            ThemeColor::Default => 0,
            ThemeColor::Red => 1,
            ThemeColor::Pink => 2,
            ThemeColor::Purple => 3,
            ThemeColor::DeepPurple => 4,
            ThemeColor::Indigo => 5,
            ThemeColor::Blue => 6,
            ThemeColor::LightBlue => 7,
            ThemeColor::Cyan => 8,
            ThemeColor::Teal => 9,
            ThemeColor::Green => 10,
            ThemeColor::LightGreen => 11,
            ThemeColor::Lime => 12,
            ThemeColor::DeepOrange => 13,
            ThemeColor::Brown => 14,
            ThemeColor::BlueGray => 15,
        };

        let code = match self {
            RawTheme::Dark(ThemeColor::Default) => 0,
            RawTheme::Dark(_) => color_code + 2,
            RawTheme::Dim => 1,
            RawTheme::Light(ThemeColor::Default) => 2,
            RawTheme::Light(_) => color_code + 17,
            RawTheme::SystemDependent(_) => color_code + 33,
            RawTheme::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i32(code)
    }
}

impl<'de> Deserialize<'de> for RawTheme {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let code = i32::deserialize(deserializer)?;

        let color_code = match code {
            3..=17 => code - 2,
            18..=32 => code - 17,
            34..=48 => code - 33,
            _ => 0,
        };
        let color = match color_code {
            1 => ThemeColor::Red,
            2 => ThemeColor::Pink,
            3 => ThemeColor::Purple,
            4 => ThemeColor::DeepPurple,
            5 => ThemeColor::Indigo,
            6 => ThemeColor::Blue,
            7 => ThemeColor::LightBlue,
            8 => ThemeColor::Cyan,
            9 => ThemeColor::Teal,
            10 => ThemeColor::Green,
            11 => ThemeColor::LightGreen,
            12 => ThemeColor::Lime,
            13 => ThemeColor::DeepOrange,
            14 => ThemeColor::Brown,
            15 => ThemeColor::BlueGray,
            _ => ThemeColor::Default,
        };

        Ok(match code {
            0 | 3..=17 => RawTheme::Dark(color),
            1 => RawTheme::Dim,
            2 | 18..=32 => RawTheme::Light(color),
            33..=48 => RawTheme::SystemDependent(color),
            other => RawTheme::Unknown(other),
        })
    }
}

impl From<RawTheme> for Theme {
    fn from(value: RawTheme) -> Self {
        match value {
            RawTheme::SystemDependent(color) => Theme::SystemDependent(color),
            RawTheme::Light(color) => Theme::Light(color),
            RawTheme::Dim => Theme::Dim,
            RawTheme::Dark(color) => Theme::Dark(color),
            RawTheme::Unknown(theme) => Theme::Unknown(theme),
        }
    }
}

impl From<Theme> for RawTheme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::SystemDependent(color) => RawTheme::SystemDependent(color),
            Theme::Light(color) => RawTheme::Light(color),
            Theme::Dim => RawTheme::Dim,
            Theme::Dark(color) => RawTheme::Dark(color),
            Theme::Unknown(unknown) => RawTheme::Unknown(unknown),
        }
    }
}
