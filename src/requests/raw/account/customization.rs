use serde::Deserialize;

use crate::models::account::Customization;
use crate::requests::raw::RawBadge;

#[derive(Deserialize)]
pub(crate) struct RawCustomization {
    #[serde(rename = "nc")]
    pub name_color: Option<i32>,
    #[serde(rename = "ab")]
    pub active_badge: Option<RawBadge>,
}

impl From<RawCustomization> for Customization {
    fn from(value: RawCustomization) -> Self {
        Self {
            name_color: value.name_color.map(i32::cast_unsigned),
            active_badge: value.active_badge.map(Into::into),
        }
    }
}
