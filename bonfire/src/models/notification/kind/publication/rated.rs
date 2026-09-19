#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::publication::PostTitle;
use crate::models::{AccountRef, PublicationRef};

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PublicationRated {
    pub amount: f64,
    pub account: Option<AccountRef>,
    pub publication: PublicationRef,
    pub post_title: Option<PostTitle>,
    pub parent: Option<PublicationRef>,
}
