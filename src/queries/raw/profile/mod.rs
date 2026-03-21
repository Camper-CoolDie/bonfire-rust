use chrono::NaiveDate;
use serde::Deserialize;
use serde::de::Error as _;

use crate::models::Profile;
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawProfile {
    pub id: String,
    #[serde(rename = "username")]
    pub name: String,
    pub email: String,
    pub cached_level: f64,
    pub birthday: Option<NaiveDate>,
    pub is_nsfw_allowed: Option<bool>,
}

impl TryFrom<RawProfile> for Profile {
    type Error = Error;

    fn try_from(value: RawProfile) -> Result<Self> {
        Ok(Self {
            // This field will always contain an integer, trust me
            id: value.id.parse().map_err(|error| {
                serde_json::Error::custom(format!("failed to convert id into u64 ({error})"))
            })?,
            name: value.name,
            email: value.email,
            cached_level: value.cached_level / 100.0,
            birthday: value.birthday,
            is_nsfw_allowed: value.is_nsfw_allowed,
        })
    }
}
