mod color;

pub use color::Color;

/// Represents the visual theme selected by the user.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Theme {
    /// The theme automatically adjusts based on the system's theme settings.
    ///
    /// The provided `Color` specifies the accent color for this theme.
    SystemDependent(Color),
    /// A light theme.
    ///
    /// The provided `Color` specifies the accent color for this theme.
    Light(Color),
    /// A dim theme
    Dim,
    /// A dark theme.
    ///
    /// The provided `Color` specifies the accent color for this theme.
    Dark(Color),
    /// An unknown or unrecognized theme variant
    Unknown(i32),
}

impl Default for Theme {
    fn default() -> Self {
        Self::SystemDependent(Color::default())
    }
}

#[cfg(feature = "serde")]
mod inner_serde {
    use std::result::Result as StdResult;

    use serde::{Deserialize, Serialize};

    use super::{Color, Theme};

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum ThemeInput {
        String(String),
        Integer(i32),
    }

    impl Serialize for Theme {
        fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            let (theme_code, color) = match self {
                Theme::SystemDependent(color) => ("system_dependent", Some(color)),
                Theme::Light(color) => ("light", Some(color)),
                Theme::Dim => ("dim", None),
                Theme::Dark(color) => ("dark", Some(color)),
                Theme::Unknown(unknown) => return serializer.serialize_i32(*unknown),
            };

            let color_code = match color {
                Some(Color::Red) => "red",
                Some(Color::Pink) => "pink",
                Some(Color::Purple) => "purple",
                Some(Color::DeepPurple) => "deep_purple",
                Some(Color::Indigo) => "indigo",
                Some(Color::Blue) => "blue",
                Some(Color::LightBlue) => "light_blue",
                Some(Color::Cyan) => "cyan",
                Some(Color::Teal) => "teal",
                Some(Color::Green) => "green",
                Some(Color::LightGreen) => "light_green",
                Some(Color::Lime) => "lime",
                Some(Color::DeepOrange) => "deep_orange",
                Some(Color::Brown) => "brown",
                Some(Color::BlueGray) => "blue_gray",
                Some(Color::Default) | None => "",
            };

            serializer.serialize_str(&(theme_code.to_owned() + color_code))
        }
    }

    impl<'de> Deserialize<'de> for Theme {
        fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            let code = match ThemeInput::deserialize(deserializer)? {
                ThemeInput::String(code) => code,
                ThemeInput::Integer(unknown) => return Ok(Theme::Unknown(unknown)),
            };
            let (theme_code, color_code) = code.split_once(':').unwrap_or((&code, ""));

            let color = match color_code {
                "red" => Color::Red,
                "pink" => Color::Pink,
                "purple" => Color::Purple,
                "deep_purple" => Color::DeepPurple,
                "indigo" => Color::Indigo,
                "blue" => Color::Blue,
                "light_blue" => Color::LightBlue,
                "cyan" => Color::Cyan,
                "teal" => Color::Teal,
                "green" => Color::Green,
                "light_green" => Color::LightGreen,
                "lime" => Color::Lime,
                "deep_orange" => Color::DeepOrange,
                "brown" => Color::Brown,
                "blue_gray" => Color::BlueGray,
                "" => Color::Default,
                unknown => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid color code: {unknown}"
                    )));
                }
            };

            Ok(match theme_code {
                "system_dependent" => Theme::SystemDependent(color),
                "light" => Theme::Light(color),
                "dim" => Theme::Dim,
                "dark" => Theme::Dark(color),
                unknown => {
                    return Err(serde::de::Error::custom(format!(
                        "invalid theme code: {unknown}"
                    )));
                }
            })
        }
    }
}
