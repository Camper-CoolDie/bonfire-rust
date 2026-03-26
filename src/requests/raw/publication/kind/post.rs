use serde::Deserialize;

use crate::models::Post;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawAccount, RawCategory, RawComment, RawFandom, RawPublication};
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
    pub fandom: RawFandom,
    #[serde(rename = "creator")]
    pub author: RawAccount,
    pub category: RawCategory,
    pub best_comment: Option<RawPublication<RawComment>>,
    #[serde(rename = "karmaCount")]
    pub karma: f64,
    pub my_karma: f64,
    #[serde(rename = "closed")]
    pub is_closed: bool,
    #[serde(rename = "subUnitsCount")]
    pub comments_count: u64,
    pub rubric_id: u64,
    pub rubric_name: String,
    #[serde(rename = "rubricKarmaCof")]
    pub rubric_karma_coef: f64,
    // #[serde(rename = "userActivity")]
    // pub relay_race: Option<RawRelayRace>,
    #[serde(rename = "important")]
    pub importance: i64,
    #[serde(rename = "blacklisted")]
    pub is_hidden: bool,
    #[serde(rename = "nsfw")]
    pub is_nsfw: bool,
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
            fandom: value.fandom.try_into()?,
            author: value.author.try_into()?,
            category: value.category.into(),
            best_comment: value.best_comment.map(TryInto::try_into).transpose()?,
            karma: value.karma / 100.0,
            my_karma: match value.my_karma {
                0.0 => None,
                karma => Some(karma / 100.0),
            },
            is_closed: value.is_closed,
            comments_count: value.comments_count,
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
            is_important: matches!(value.importance, -1),
            is_hidden: value.is_hidden,
            is_nsfw: value.is_nsfw,
        })
    }
}
