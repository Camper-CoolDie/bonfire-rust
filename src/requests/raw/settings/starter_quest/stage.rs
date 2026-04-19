use std::result::Result as StdResult;

use serde::{Deserialize, Serialize};

use crate::models::settings::StarterQuestStage;

#[derive(Debug)]
pub(crate) enum RawStage {
    Intro,
    RatePublications,
    OpenAchievements,
    CreateChatMessages,
    SubscribeToFandom,
    OpenProfile,
    CreateComments,
    OpenFeedFilters,
    AddToFavorites,
    OpenFavorites,
    CreatePostDraft,
    OpenLeaderboard,
    AddStickerPackToCollection,
    PublishPost,
    Complete,
    Completed,
    Unknown(i64),
}

impl Serialize for RawStage {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let stage = match self {
            RawStage::Intro => 0,
            RawStage::RatePublications => 1,
            RawStage::OpenAchievements => 2,
            RawStage::CreateChatMessages => 3,
            RawStage::SubscribeToFandom => 4,
            RawStage::OpenProfile => 5,
            RawStage::CreateComments => 6,
            RawStage::OpenFeedFilters => 7,
            RawStage::AddToFavorites => 8,
            RawStage::OpenFavorites => 9,
            RawStage::CreatePostDraft => 10,
            RawStage::OpenLeaderboard => 11,
            RawStage::AddStickerPackToCollection => 12,
            RawStage::PublishPost => 13,
            RawStage::Complete => 14,
            RawStage::Completed => 15,
            RawStage::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(stage)
    }
}

impl<'de> Deserialize<'de> for RawStage {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            0 => RawStage::Intro,
            1 => RawStage::RatePublications,
            2 => RawStage::OpenAchievements,
            3 => RawStage::CreateChatMessages,
            4 => RawStage::SubscribeToFandom,
            5 => RawStage::OpenProfile,
            6 => RawStage::CreateComments,
            7 => RawStage::OpenFeedFilters,
            8 => RawStage::AddToFavorites,
            9 => RawStage::OpenFavorites,
            10 => RawStage::CreatePostDraft,
            11 => RawStage::OpenLeaderboard,
            12 => RawStage::AddStickerPackToCollection,
            13 => RawStage::PublishPost,
            14 => RawStage::Complete,
            15 => RawStage::Completed,
            other => RawStage::Unknown(other),
        })
    }
}

impl From<RawStage> for Option<StarterQuestStage> {
    fn from(value: RawStage) -> Self {
        match value {
            RawStage::Completed => None,
            RawStage::Intro => Some(StarterQuestStage::Intro),
            RawStage::RatePublications => Some(StarterQuestStage::RatePublications),
            RawStage::OpenAchievements => Some(StarterQuestStage::OpenAchievements),
            RawStage::CreateChatMessages => Some(StarterQuestStage::CreateChatMessages),
            RawStage::SubscribeToFandom => Some(StarterQuestStage::SubscribeToFandom),
            RawStage::OpenProfile => Some(StarterQuestStage::OpenProfile),
            RawStage::CreateComments => Some(StarterQuestStage::CreateComments),
            RawStage::OpenFeedFilters => Some(StarterQuestStage::OpenFeedFilters),
            RawStage::AddToFavorites => Some(StarterQuestStage::AddToFavorites),
            RawStage::OpenFavorites => Some(StarterQuestStage::OpenFavorites),
            RawStage::CreatePostDraft => Some(StarterQuestStage::CreatePostDraft),
            RawStage::OpenLeaderboard => Some(StarterQuestStage::OpenLeaderboard),
            RawStage::AddStickerPackToCollection => {
                Some(StarterQuestStage::AddStickerPackToCollection)
            }
            RawStage::PublishPost => Some(StarterQuestStage::PublishPost),
            RawStage::Complete => Some(StarterQuestStage::Complete),
            RawStage::Unknown(stage) => Some(StarterQuestStage::Unknown(stage)),
        }
    }
}

impl From<StarterQuestStage> for RawStage {
    fn from(value: StarterQuestStage) -> Self {
        match value {
            StarterQuestStage::Intro => RawStage::Intro,
            StarterQuestStage::RatePublications => RawStage::RatePublications,
            StarterQuestStage::OpenAchievements => RawStage::OpenAchievements,
            StarterQuestStage::CreateChatMessages => RawStage::CreateChatMessages,
            StarterQuestStage::SubscribeToFandom => RawStage::SubscribeToFandom,
            StarterQuestStage::OpenProfile => RawStage::OpenProfile,
            StarterQuestStage::CreateComments => RawStage::CreateComments,
            StarterQuestStage::OpenFeedFilters => RawStage::OpenFeedFilters,
            StarterQuestStage::AddToFavorites => RawStage::AddToFavorites,
            StarterQuestStage::OpenFavorites => RawStage::OpenFavorites,
            StarterQuestStage::CreatePostDraft => RawStage::CreatePostDraft,
            StarterQuestStage::OpenLeaderboard => RawStage::OpenLeaderboard,
            StarterQuestStage::AddStickerPackToCollection => RawStage::AddStickerPackToCollection,
            StarterQuestStage::PublishPost => RawStage::PublishPost,
            StarterQuestStage::Complete => RawStage::Complete,
            StarterQuestStage::Unknown(unknown) => RawStage::Unknown(unknown),
        }
    }
}
