use serde::Deserialize;

use crate::requests::raw::RawBadge;

#[derive(Deserialize)]
pub(crate) struct RawCustomization {
    #[serde(rename = "nc")]
    pub name_color: Option<i32>,
    #[serde(rename = "ab")]
    pub active_badge: Option<RawBadge>,
}
