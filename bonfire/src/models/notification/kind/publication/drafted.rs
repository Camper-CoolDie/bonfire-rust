#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::Gender;
use crate::models::publication::{Kind, PostTitle};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PublicationDrafted {
    pub kind: Kind,
    pub title: Option<PostTitle>,
    pub moderation_id: u64,
    pub moderator_name: String,
    pub moderator_gender: Gender,
    pub reason: String,
}
