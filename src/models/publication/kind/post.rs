#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::client::Request as _;
use crate::models::publication::{Kind, PostTag, Publishable};
use crate::models::{Account, Category, Comment, Fandom, Publication};
use crate::requests::publication::post::GetPostRequest;
use crate::sealed::Sealed;
use crate::{Client, Result};

/// Represents the specific data for a post publication.
#[derive(Default, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]
pub struct Post {
    /// The fandom in which this publication was posted
    pub fandom: Fandom,
    /// The account that authored this publication
    pub author: Account,
    /// The specific category of the fandom in which this publication was posted
    pub category: Category,
    // /// The content of this post
    // pub pages: Vec<Page>,
    /// A comment which earned the highest amount of karma
    pub best_comment: Option<Publication<Comment>>,
    /// The total karma received by this publication (can be positive or negative)
    pub karma: f64,
    /// The karma you personally placed on this publication, if any
    pub my_karma: Option<f64>,
    /// Indicates if this publication will appear in the main feed
    pub is_closed: bool,
    /// The total number of comments associated with this publication
    pub comments_count: u64,
    /// The identifier of the rubric this post is linked to
    pub rubric_id: Option<u64>,
    /// The name of the rubric this post is linked to
    pub rubric_name: Option<String>,
    /// The karma coefficient of the rubric this post is linked to
    pub rubric_karma_coef: Option<f64>,
    // /// A relay race this post is linked to
    // pub relay_race: Option<RelayRace>,
    /// Indicates if this publication has been marked as important
    pub is_important: bool,
    /// Indicates if this publication originates from a blocked fandom or account
    pub is_hidden: bool,
    /// Indicates if this publication has been marked as NSFW
    pub is_nsfw: bool,
}

impl Publishable for Post {
    /// Returns the publication kind as [`Kind::Post`].
    fn kind(&self) -> Kind {
        Kind::Post
    }
}

impl Sealed for Post {}

impl Publication<Post> {
    /// Retrieves a post and its tags by the post's unique identifier. If the caller is the post's
    /// creator, a draft/pending post can also be retrieved.
    ///
    /// The `Vec<Publication<PostTag>>` returned will contain both tag categories and individual
    /// tags. Categories do not have a parent ID, while tags will always reference their parent
    /// category. The list is sorted by categories first, then by tags within each category. For
    /// example: `Category #1`, `Tag #1` (under Category #1), `Category #2`, `Tag #2` (under
    /// Category #2), `Tag #3` (under Category #2).
    ///
    /// # Errors
    ///
    /// * Returns [`UnavailableError::Removed`][crate::UnavailableError::Removed] if no post with
    ///   the provided identifier exists or it is removed.
    /// * Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if a post with
    ///   the provided identifier is unavailable (e.g., a draft/pending post not owned by the
    ///   caller).
    /// * Returns [`UnavailableError::Blocked`][crate::UnavailableError::Blocked] if the post is
    ///   blocked.
    /// * Returns [`Error`][crate::Error] if any other error occurs during the request.
    pub async fn get_by_id(client: &Client, id: u64) -> Result<(Self, Vec<Publication<PostTag>>)> {
        GetPostRequest::new(id)
            .send_request(client)
            .await?
            .try_into()
    }
}
