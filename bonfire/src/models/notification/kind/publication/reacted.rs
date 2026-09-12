#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, PublicationRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Reacted {
    pub index: i64,
    pub account: AccountRef,
    pub publication: PublicationRef,
    pub parent: Option<PublicationRef>,
}
