use serde::Deserialize;

use crate::models::account::AccountCustomization;
use crate::requests::raw::RawBadge;

#[derive(Deserialize)]
pub(crate) struct RawAccountCustomization {
    #[serde(rename = "nc")]
    name_color: Option<i32>,
    #[serde(rename = "ab")]
    active_badge: Option<RawBadge>,
}

impl From<RawAccountCustomization> for AccountCustomization {
    fn from(value: RawAccountCustomization) -> Self {
        Self {
            name_color: value.name_color.map(i32::cast_unsigned),
            active_badge: value.active_badge.map(Into::into),
        }
    }
}
