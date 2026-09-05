#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use super::ItemKind;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Title {
    Text { text: String, is_truncated: bool },
    Other { item_kind: ItemKind },
}
