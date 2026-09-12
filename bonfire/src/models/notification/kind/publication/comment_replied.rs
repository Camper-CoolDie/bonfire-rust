#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::{AccountRef, PublicationRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct CommentReplied {
    pub id: u64,
    pub author: AccountRef,
    pub parent: PublicationRef,
    pub text: Option<String>,
}
