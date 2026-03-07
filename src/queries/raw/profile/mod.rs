use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::Profile;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawProfile {
    pub id: String,
    #[serde(rename = "username")]
    name: String,
    email: String,
    cached_level: f64,
    birthday: Option<NaiveDate>,
    is_nsfw_allowed: Option<bool>,
}

impl From<RawProfile> for Profile {
    fn from(value: RawProfile) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            cached_level: value.cached_level / 100.,
            birthday: value.birthday,
            is_nsfw_allowed: value.is_nsfw_allowed,
        }
    }
}
