use std::collections::VecDeque;
use std::result::Result as StdResult;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize};

#[cfg(feature = "serde")]
fn deserialize_persistent_ids<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> StdResult<VecDeque<String>, D::Error> {
    let mut ids = VecDeque::deserialize(deserializer)?;
    ids.reserve_exact(Subscription::PERSISTENT_IDS_MAX_COUNT);
    Ok(ids)
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Subscription {
    // Used only in logs to keep track of multiple listeners, usually set to the account's
    // identifier which will have the push token
    pub id: u64,
    pub push_token: String,
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "deserialize_persistent_ids")
    )]
    pub persistent_ids: VecDeque<String>,
}
impl Subscription {
    // https://firebase.google.com/docs/cloud-messaging/android/receive-messages#override-on-deleted-messages
    pub(crate) const PERSISTENT_IDS_MAX_COUNT: usize = 100;
}
