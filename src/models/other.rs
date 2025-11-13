use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Represents a reference to an external image.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct ImageRef {
    /// The image's URI. Empty if the image is unset or default
    #[serde(rename = "u")]
    pub uri: String,
    /// The image's width. Can be zero
    #[serde(rename = "w")]
    pub width: i32,
    /// The image's height. Can be zero
    #[serde(rename = "h")]
    pub height: i32,
    /// A unique identifier of the image. Zero if the image is unset or default
    #[serde(rename = "i")]
    pub id: i64,
}
impl ImageRef {
    pub(crate) fn serialize_or_none<S: Serializer>(
        value: &Option<ImageRef>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            None => ImageRef::default(),
            Some(value) => value.clone(),
        }
        .serialize(serializer)
    }

    pub(crate) fn deserialize_or_none<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<ImageRef>, D::Error> {
        let value = ImageRef::deserialize(deserializer)?;
        Ok(match value.id {
            0 => None,
            _ => Some(value),
        })
    }
}
