use serde::Deserialize;

use crate::models::account::Stat;
use crate::models::publication::PublicationKind;

#[derive(Deserialize)]
pub(crate) struct RawStat {
    #[serde(rename = "totalRatesPlus")]
    pub positive_rates_sum: f64,
    #[serde(rename = "totalRatesMinus")]
    pub negative_rates_sum: f64,
    #[serde(rename = "totalKarmaPlus")]
    pub others_positive_rates_sum: f64,
    #[serde(rename = "totalKarmaMinus")]
    pub others_negative_rates_sum: f64,
    #[serde(rename = "totalPosts")]
    pub posts_count: u64,
    #[serde(rename = "totalComments")]
    pub comments_count: u64,
    #[serde(rename = "totalMessages")]
    pub messages_count: u64,
    #[serde(rename = "bestPost")]
    pub best_post_id: u64,
    #[serde(rename = "bestComment")]
    pub best_comment_id: u64,
    #[serde(rename = "bestCommentUnitId")]
    pub best_comment_parent_id: u64,
    #[serde(rename = "bestCommentUnitType")]
    pub best_comment_parent_kind: i64,
}

impl From<RawStat> for Stat {
    fn from(value: RawStat) -> Self {
        let best_comment_parent_kind =
            (value.best_comment_id != 0).then_some(match value.best_comment_parent_kind {
                1 => PublicationKind::Comment,
                8 => PublicationKind::ChatMessage,
                9 => PublicationKind::Post,
                10 => PublicationKind::PostTag,
                11 => PublicationKind::Moderation,
                12 => PublicationKind::UserEvent,
                15 => PublicationKind::StickerPack,
                16 => PublicationKind::Sticker,
                17 => PublicationKind::ModerationEvent,
                18 => PublicationKind::AdminEvent,
                19 => PublicationKind::FandomEvent,
                21 => PublicationKind::Quest,
                _ => PublicationKind::Unknown,
            });

        Self {
            positive_rates_sum: value.positive_rates_sum / 100.,
            negative_rates_sum: value.negative_rates_sum / 100.,
            others_positive_rates_sum: value.others_positive_rates_sum / 100.,
            others_negative_rates_sum: value.others_negative_rates_sum / 100.,
            posts_count: value.posts_count,
            comments_count: value.comments_count,
            messages_count: value.messages_count,
            best_post_id: match value.best_post_id {
                0 => None,
                _ => Some(value.best_post_id),
            },
            best_comment_id: match value.best_comment_id {
                0 => None,
                _ => Some(value.best_comment_id),
            },
            best_comment_parent_id: match value.best_comment_id {
                0 => None,
                _ => Some(value.best_comment_parent_id),
            },
            best_comment_parent_kind,
        }
    }
}
