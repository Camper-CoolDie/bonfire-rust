use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::ChatTag;
use crate::requests::raw::RawLanguage;
use crate::requests::raw::chat::RawKind;
use crate::{Error, Result};

// The server always accepts and returns tags in the "<kind>-<first_id>-<second_id>" format. Since
// second_id may be either an account's ID (u64) or a language ID (i64), our serialization logic is
// a bit complex
pub(crate) enum RawTag {
    FandomRoot {
        fandom_id: u64,
        language: RawLanguage,
    },
    FandomSub {
        id: u64,
    },
    Group {
        id: u64,
    },
    Direct {
        my_id: u64,
        partner_id: u64,
    },
    Unknown((i64, u64, u64)),
}

impl Serialize for RawTag {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let kind: &str = &i64::from(RawKind::from(self)).to_string();

        let (first_id, second_id): (&str, &str) = match self {
            RawTag::FandomRoot {
                fandom_id,
                language,
            } => (&fandom_id.to_string(), &i64::from(language).to_string()),
            RawTag::FandomSub { id } | RawTag::Group { id } => (&id.to_string(), &0.to_string()),
            RawTag::Direct { my_id, partner_id } => (&my_id.to_string(), &partner_id.to_string()),
            RawTag::Unknown((_, first_id, second_id)) => {
                (&first_id.to_string(), &second_id.to_string())
            }
        };

        serializer.serialize_str(&[kind, first_id, second_id].join("-"))
    }
}

impl<'de> Deserialize<'de> for RawTag {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let tag = String::deserialize(deserializer)?;
        let parts = tag.split('-').collect::<Vec<_>>();
        let [kind, first_id, second_id] = parts
            .get(0..3)
            .ok_or(serde::de::Error::custom("invalid chat tag format"))?
        else {
            return Err(serde::de::Error::custom("chat tag has extra parts"));
        };

        let kind = RawKind::from(kind.parse::<i64>().map_err(|error| {
            serde::de::Error::custom(format!("failed to convert tag kind into i64 ({error})"))
        })?);

        let first_id = first_id.parse::<u64>().map_err(|error| {
            serde::de::Error::custom(format!("failed to convert tag first_id into u64 ({error})"))
        })?;
        let (second_id, language) = if matches!(kind, RawKind::FandomRoot) {
            let language = RawLanguage::from(second_id.parse::<i64>().map_err(|error| {
                serde::de::Error::custom(format!(
                    "failed to convert tag second_id into i64 ({error})"
                ))
            })?);
            (0, language)
        } else {
            let second_id = second_id.parse::<u64>().map_err(|error| {
                serde::de::Error::custom(format!(
                    "failed to convert tag second_id into u64 ({error})"
                ))
            })?;
            (second_id, RawLanguage::Unknown(0))
        };

        Ok(match kind {
            RawKind::FandomRoot => RawTag::FandomRoot {
                fandom_id: first_id,
                language,
            },
            RawKind::FandomSub => RawTag::FandomSub { id: first_id },
            RawKind::Group => RawTag::Group { id: first_id },
            RawKind::Direct => RawTag::Direct {
                my_id: first_id,
                partner_id: second_id,
            },
            RawKind::Unknown(unknown) => RawTag::Unknown((unknown, first_id, second_id)),
        })
    }
}

impl TryFrom<RawTag> for ChatTag {
    type Error = Error;

    fn try_from(value: RawTag) -> Result<Self> {
        Ok(match value {
            RawTag::FandomRoot {
                fandom_id,
                language,
            } => ChatTag::FandomRoot {
                fandom_id,
                language: language.try_into()?,
            },
            RawTag::FandomSub { id } => ChatTag::FandomSub { id },
            RawTag::Group { id } => ChatTag::Group { id },
            RawTag::Direct { my_id, partner_id } => ChatTag::Direct { my_id, partner_id },
            RawTag::Unknown((unknown, _, _)) => return Err(Error::UnknownVariant(unknown)),
        })
    }
}

impl From<&RawTag> for RawKind {
    fn from(value: &RawTag) -> Self {
        match value {
            RawTag::FandomRoot { .. } => RawKind::FandomRoot,
            RawTag::FandomSub { .. } => RawKind::FandomSub,
            RawTag::Group { .. } => RawKind::Group,
            RawTag::Direct { .. } => RawKind::Direct,
            RawTag::Unknown((unknown, _, _)) => RawKind::Unknown(*unknown),
        }
    }
}

impl From<ChatTag> for RawTag {
    fn from(value: ChatTag) -> Self {
        match value {
            ChatTag::FandomRoot {
                fandom_id,
                language,
            } => RawTag::FandomRoot {
                fandom_id,
                language: language.into(),
            },
            ChatTag::FandomSub { id } => RawTag::FandomSub { id },
            ChatTag::Group { id } => RawTag::Group { id },
            ChatTag::Direct { my_id, partner_id } => RawTag::Direct { my_id, partner_id },
        }
    }
}
