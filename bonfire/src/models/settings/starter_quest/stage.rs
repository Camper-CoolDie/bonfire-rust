#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Represents a stage in the starter quest.
///
/// Stages are sorted in the order they should appear to the user.
#[derive(Default, Clone, Debug, PartialEq, Eq, EnumIter)]
#[cfg_attr(
    feature = "serde",
    derive(Deserialize, Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Stage {
    /// The initial stage
    #[default]
    Intro,
    /// Stage for rating publications
    RatePublications,
    /// Stage for opening achievements
    OpenAchievements,
    /// Stage for creating chat messages
    CreateChatMessages,
    /// Stage for subscribing to a fandom
    SubscribeToFandom,
    /// Stage for opening the user profile
    OpenProfile,
    /// Stage for creating comments
    CreateComments,
    /// Stage for opening feed filters
    OpenFeedFilters,
    /// Stage for adding posts to favorites
    AddToFavorites,
    /// Stage for opening favorites
    OpenFavorites,
    /// Stage for creating a post draft
    CreatePostDraft,
    /// Stage for opening the leaderboard
    OpenLeaderboard,
    /// Stage for adding a sticker pack to the collection
    AddStickerPackToCollection,
    /// Stage for publishing a post
    PublishPost,
    /// The completion stage
    Complete,
    /// An unknown stage
    #[cfg_attr(feature = "serde", serde(untagged))]
    Unknown(i64),
}
impl Stage {
    pub(super) fn max_progress(&self) -> Option<usize> {
        match self {
            Stage::CreateChatMessages | Stage::CreateComments => Some(5),
            Stage::RatePublications | Stage::CreatePostDraft => Some(3),
            Stage::OpenAchievements
            | Stage::SubscribeToFandom
            | Stage::OpenProfile
            | Stage::OpenFeedFilters
            | Stage::AddToFavorites
            | Stage::OpenFavorites
            | Stage::OpenLeaderboard
            | Stage::AddStickerPackToCollection
            | Stage::PublishPost => Some(1),
            Stage::Intro | Stage::Complete => Some(0),
            Stage::Unknown(_) => None,
        }
    }
}
