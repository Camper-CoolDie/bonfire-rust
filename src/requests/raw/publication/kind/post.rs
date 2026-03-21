use serde::Deserialize;

use crate::models::Post;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawComment, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct InnerData {
    // #[serde(rename = "J_PAGES")]
    // pub pages: Vec<RawPage>,
    // pub title: Option<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPost {
    pub best_comment: Option<RawPublication<RawComment>>,
    pub rubric_id: u64,
    pub rubric_name: String,
    #[serde(rename = "rubricKarmaCof")]
    pub rubric_karma_coef: f64,
    // #[serde(rename = "userActivity")]
    // pub relay_race: Option<RawRelayRace>,
    #[serde(rename = "jsonDB")]
    pub inner: InnerData,
}

impl RawPublishable for RawPost {
    type Target = Post;

    fn new(data: serde_json::Value, _kind: RawKind) -> Result<Self> {
        Ok(serde_json::from_value::<RawPost>(data)?)
    }
}

impl TryFrom<RawPost> for Post {
    type Error = Error;

    fn try_from(value: RawPost) -> Result<Self> {
        Ok(Self {
            best_comment: value.best_comment.map(TryInto::try_into).transpose()?,
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
        })
    }
}
