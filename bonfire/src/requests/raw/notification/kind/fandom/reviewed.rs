use serde::Deserialize;

use crate::models::notification::FandomReviewed;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawReviewed {
    #[serde(rename = "accepted")]
    pub is_accepted: bool,
    pub fandom_id: u64,
    pub fandom_name: String,
    pub admin_name: String,
    #[serde(rename = "comment")]
    pub note: String,
}

impl From<RawReviewed> for FandomReviewed {
    fn from(value: RawReviewed) -> Self {
        Self {
            is_accepted: value.is_accepted,
            fandom_id: value.fandom_id,
            fandom_name: value.fandom_name,
            admin_name: value.admin_name,
            note: value.note,
        }
    }
}
