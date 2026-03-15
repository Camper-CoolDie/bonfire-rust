use serde::Deserialize;

use crate::models::Post;
use crate::requests::raw::publication::{RawPublicationInheritor, RawPublicationKind};
use crate::{Error, Result};

#[derive(Deserialize)]
struct InnerData {
    // #[serde(rename = "J_PAGES")]
    // pages: Vec<RawPage>,
    // best_comment: Option<RawPublication<Comment>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPost {
    rubric_id: u64,
    rubric_name: String,
    #[serde(rename = "rubricKarmaCof")]
    rubric_karma_coef: f64,
    // #[serde(rename = "userActivity")]
    // relay_race: Option<RawRelayRace>,
    #[serde(rename = "jsonDB")]
    inner: InnerData,
}

impl RawPublicationInheritor for RawPost {
    type Target = Post;

    fn new(data: serde_json::Value, _kind: RawPublicationKind) -> Result<Self> {
        Ok(serde_json::from_value::<RawPost>(data)?)
    }
}

impl TryFrom<RawPost> for Post {
    type Error = Error;

    fn try_from(value: RawPost) -> Result<Self> {
        Ok(Self {
            // pages: value.inner.pages.map(TryInto::try_into).collect()?,
            // best_comment: value.inner.best_comment.map(TryInto::try_into).transpose()?,
            rubric_id: match value.rubric_id {
                0 => None,
                id => Some(id),
            },
            rubric_name: match value.rubric_id {
                0 => None,
                _ => Some(value.rubric_name),
            },
            rubric_karma_coef: match value.rubric_id {
                0 => None,
                _ => Some(value.rubric_karma_coef / 100.0),
            },
            // relay_race: value.relay_race.map(TryInto::try_into).transpose()?,
        })
    }
}
