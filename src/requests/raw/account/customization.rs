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
        // The server uses i32 to represent colors
        #[expect(clippy::cast_sign_loss)]
        Self {
            name_color: value.name_color.map(|color| color as u32),
            active_badge: value.active_badge.map(Into::into),
        }
    }
}
