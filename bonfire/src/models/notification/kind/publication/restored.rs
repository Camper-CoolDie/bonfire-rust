#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::models::PublicationRef;

#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct PublicationRestored {
    pub publication: PublicationRef,
    pub parent: PublicationRef,
    pub reason: String,
}
