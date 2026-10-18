#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::ImageRef;

#[derive(Clone, Debug)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Content {
    Normal {
        image: ImageRef,
    },
    Gif {
        first_frame: ImageRef,
        animated: ImageRef,
    },
}

impl Default for Content {
    fn default() -> Self {
        Content::Normal {
            image: ImageRef::default(),
        }
    }
}
