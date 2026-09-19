#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::PostTitle;
use crate::models::{AccountRef, PublicationRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PublicationCommented {
    pub id: u64,
    pub author: AccountRef,
    pub fandom_name: Option<String>,
    pub parent: PublicationRef,
    pub parent_author_id: u64,
    pub parent_post_title: Option<PostTitle>,
    pub text: Option<String>,
}
