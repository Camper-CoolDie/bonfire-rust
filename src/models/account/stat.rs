use crate::client::Request as _;
use crate::models::publication::PublicationKind;
use crate::requests::account::GetStatRequest;
use crate::{Client, Result};

/// Represents an account's overall statistics.
#[derive(Default, Clone, Debug)]
pub struct Stat {
    /// The sum of all positive rates placed by this account
    pub positive_rates_sum: f64,
    /// The sum of all negative rates placed by this account
    pub negative_rates_sum: f64,
    /// The sum of all positive rates placed by others on this account's publications
    pub others_positive_rates_sum: f64,
    /// The sum of all negative rates placed by others on this account's publications
    pub others_negative_rates_sum: f64,
    /// The total number of posts created by this account
    pub posts_count: u64,
    /// The total number of comments made by this account
    pub comments_count: u64,
    /// The total number of private messages sent by this account
    pub messages_count: u64,
    /// The ID of this account's highest-rated post
    pub best_post_id: Option<u64>,
    /// The ID of this account's highest-rated comment
    pub best_comment_id: Option<u64>,
    /// The ID of the parent publication of this account's highest-rated comment
    pub best_comment_parent_id: Option<u64>,
    /// The type of the parent publication of this account's highest-rated comment
    pub best_comment_parent_kind: Option<PublicationKind>,
}
impl Stat {
    /// Retrieves an account's statistics by its unique identifier. If no account with the provided
    /// identifier exists, this method returns a default, empty `Stat`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn get(client: &Client, id: u64) -> Result<Self> {
        Ok(GetStatRequest::new(id).send_request(client).await?.into())
    }
}
