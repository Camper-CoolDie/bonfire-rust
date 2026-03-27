use std::result::Result as StdResult;

use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize};

use crate::models::ChatTag;
use crate::requests::raw::RawLanguage;
use crate::requests::raw::chat::RawKind;
use crate::{Error, Result};

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
        recipient_id: u64,
    },
    Unknown {
        kind: i64,
        first_id: u64,
        second_id: u64,
    },
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawTagInput {
    Map {
        #[serde(rename = "chatType")]
        kind: RawKind,
        #[serde(rename = "targetId")]
        first_id: u64,
        #[serde(rename = "targetSubId")]
        second_id: u64,
    },
    String(String),
}

impl Serialize for RawTag {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (kind, first_id, second_id) = self.into();

        let mut tag = serializer.serialize_struct("RawChatTag", 3)?;
        tag.serialize_field("chatType", &i64::from(kind))?;
        tag.serialize_field("targetId", &first_id)?;
        tag.serialize_field("targetSubId", &second_id)?;
        tag.end()
    }
}

impl<'de> Deserialize<'de> for RawTag {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let parts = match RawTagInput::deserialize(deserializer)? {
            RawTagInput::Map {
                kind,
                first_id,
                second_id,
            } => (kind, first_id, second_id),
            RawTagInput::String(tag) => {
                let parts = tag.split('-').collect::<Vec<_>>();
                if parts.len() != 3 {
                    return Err(serde::de::Error::custom("invalid chat tag format"));
                }

                let kind = parts[0].parse::<i64>().map_err(|error| {
                    serde::de::Error::custom(format!(
                        "failed to convert tag kind into i64: {error}"
                    ))
                })?;
                let first_id = parts[1].parse::<u64>().map_err(|error| {
                    serde::de::Error::custom(format!(
                        "failed to convert tag first_id into u64: {error}"
                    ))
                })?;
                let second_id = parts[2].parse::<u64>().map_err(|error| {
                    serde::de::Error::custom(format!(
                        "failed to convert tag second_id into u64: {error}"
                    ))
                })?;

                (RawKind::from(kind), first_id, second_id)
            }
        };

        Ok(parts.into())
    }
}

impl From<&RawTag> for (RawKind, u64, u64) {
    fn from(value: &RawTag) -> Self {
        match value {
            RawTag::FandomRoot {
                fandom_id,
                language,
            } => (RawKind::FandomRoot, *fandom_id, u64::from(language)),
            RawTag::FandomSub { id } => (RawKind::FandomSub, *id, 0),
            RawTag::Group { id } => (RawKind::Group, *id, 0),
            RawTag::Direct {
                my_id,
                recipient_id,
            } => (RawKind::Direct, *my_id, *recipient_id),
            RawTag::Unknown {
                kind,
                first_id,
                second_id,
            } => (RawKind::Unknown(*kind), *first_id, *second_id),
        }
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
            RawTag::Direct {
                my_id,
                recipient_id,
            } => ChatTag::Direct {
                my_id,
                recipient_id,
            },
            RawTag::Unknown { kind: unknown, .. } => {
                return Err(Error::UnknownVariant(Box::new(unknown)));
            }
        })
    }
}

impl From<RawTag> for RawKind {
    fn from(value: RawTag) -> Self {
        match value {
            RawTag::FandomRoot { .. } => RawKind::FandomRoot,
            RawTag::FandomSub { .. } => RawKind::FandomSub,
            RawTag::Group { .. } => RawKind::Group,
            RawTag::Direct { .. } => RawKind::Direct,
            RawTag::Unknown { kind: unknown, .. } => RawKind::Unknown(unknown),
        }
    }
}

impl From<(RawKind, u64, u64)> for RawTag {
    fn from(value: (RawKind, u64, u64)) -> Self {
        match value.0 {
            RawKind::FandomRoot => RawTag::FandomRoot {
                fandom_id: value.1,
                language: value.2.into(),
            },
            RawKind::FandomSub => RawTag::FandomSub { id: value.1 },
            RawKind::Group => RawTag::Group { id: value.1 },
            RawKind::Direct => RawTag::Direct {
                my_id: value.1,
                recipient_id: value.2,
            },
            RawKind::Unknown(unknown) => RawTag::Unknown {
                kind: unknown,
                first_id: value.1,
                second_id: value.2,
            },
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
            ChatTag::Direct {
                my_id,
                recipient_id,
            } => RawTag::Direct {
                my_id,
                recipient_id,
            },
        }
    }
}
