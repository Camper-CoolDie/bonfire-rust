use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::Me;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawMe {
    pub id: String,
    #[serde(rename = "username")]
    name: String,
    email: String,
    cached_level: i64,
    birthday: Option<NaiveDate>,
    is_nsfw_allowed: Option<bool>,
}

impl From<RawMe> for Me {
    fn from(value: RawMe) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email,
            cached_level: value.cached_level as f32 / 100.,
            birthday: value.birthday,
            is_nsfw_allowed: value.is_nsfw_allowed,
        }
    }
}
