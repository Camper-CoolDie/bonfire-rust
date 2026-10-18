mod content;
mod error;
mod pack;

pub use content::Content;
pub use error::*;
pub use pack::Pack;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Account;
use crate::models::publication::{Kind, Publishable, Removable};
use crate::sealed::Sealed;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Sticker {
    pub content: Content,
    pub author: Account,
    pub pack_id: u64,
}

impl Publishable for Sticker {
    /// Returns the publication kind as [`Kind::Sticker`].
    fn kind(&self) -> Kind {
        Kind::Sticker
    }
}

impl Removable for Sticker {}
impl Sealed for Sticker {}
