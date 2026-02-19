use chrono::{DateTime, Utc};

use crate::client::Request as _;
use crate::models::{Category, ImageRef, Language};
use crate::requests::fandom::blocklist::{
    BlockFandomRequest, CheckFandomBlockedRequest, UnblockFandomRequest,
};
use crate::requests::fandom::{GetFandomRequest, GetFandomsRequest};
use crate::{Client, Result};

/// Represents the current status of a fandom.
#[derive(Default, Clone, Debug)]
pub enum FandomStatus {
    /// The fandom's status is unspecified
    #[default]
    Unspecified,
    /// The fandom has been suggested and is awaiting approval
    Suggested,
    /// The fandom has been accepted
    Accepted,
}

/// Represents a fandom, which is a community centered around a specific topic.
#[derive(Default, Clone, Debug)]
pub struct Fandom {
    /// The unique identifier of this fandom
    pub id: u64,
    /// The specific language for this fandom. A single fandom can exist in multiple languages,
    /// with each `Language` representing a distinct community instance. `None` if this fandom
    /// instance isn't linked to a specific language
    pub language: Option<Language>,
    /// The icon image representing this fandom
    pub icon: Option<ImageRef>,
    /// The background image set for this fandom's profile
    pub background: Option<ImageRef>,
    /// The GIF background image set for this fandom's profile
    pub background_gif: Option<ImageRef>,
    /// Indicates if publications from this fandom can randomly appear in the feed
    pub is_closed: bool,
    /// The karma coefficient applied to this fandom
    pub karma_coef: f64,
    /// The identifier of the account that originally suggested this fandom
    pub suggester_id: Option<u64>,
    /// The date and time when this fandom was suggested
    pub suggested_at: Option<DateTime<Utc>>,
    /// The number of users currently subscribed to this fandom
    pub subscribers_count: u64,
    /// The current status of this fandom
    pub status: FandomStatus,
    /// The category that this fandom belongs to
    pub category: Category,
}
impl Fandom {
    /// Creates a new `Fandom` instance with only its identifier and language set.
    ///
    /// This is useful when you only need to reference a fandom by its ID and language for sending
    /// associated requests. However, obtaining a fully populated `Fandom` struct from methods like
    /// [`Fandom::get()`] or [`Fandom::get_by_id()`] is generally preferred.
    #[must_use]
    pub fn new(id: u64, language: Language) -> Self {
        Self {
            id,
            language: Some(language),
            ..Self::default()
        }
    }

    /// Creates a new `Fandom` instance with only its identifier set.
    ///
    /// This is useful when you only need to reference a fandom by its ID for sending associated
    /// requests. However, obtaining a fully populated `Fandom` struct from methods like
    /// [`Fandom::get()`] or [`Fandom::get_by_id()`] is generally preferred.
    #[must_use]
    pub fn new_by_id(id: u64) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }

    /// Retrieves a fandom by its unique identifier and specified language.
    ///
    /// The returned [`Fandom::language`] will be the same as the `language` parameter passed to
    /// this method.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no fandom with
    /// the provided identifier exists, or [`Error`][crate::Error] if any other error occurs during
    /// the request.
    pub async fn get(
        client: &Client,
        id: u64,
        language: Option<Language>,
        my_language: Language,
    ) -> Result<Self> {
        GetFandomRequest::new(id, language, my_language)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves a single fandom by its unique identifier.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if no fandom with
    /// the provided identifier exists, or [`Error`][crate::Error] if any other error occurs during
    /// the request.
    pub async fn get_by_id(client: &Client, id: u64) -> Result<Self> {
        GetFandomsRequest::new_single(id)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves multiple fandoms by their unique identifiers.
    ///
    /// If a fandom for a given ID is not found, its corresponding entry in the returned vector will
    /// be `None`. The returned vector is guaranteed to be sorted by the order of `ids`.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs during the request.
    pub async fn get_by_ids(client: &Client, ids: Vec<u64>) -> Result<Vec<Option<Self>>> {
        GetFandomsRequest::new(ids)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Retrieves multiple fandoms by their unique identifiers.
    ///
    /// If any fandom for a given ID is not found, the entire request will fail and return an
    /// error. The returned vector is guaranteed to be sorted by the order of `ids`.
    ///
    /// # Errors
    ///
    /// Returns [`UnavailableError::NotFound`][crate::UnavailableError::NotFound] if at least one
    /// fandom for a given ID is not found, or [`Error`][crate::Error] if any other error occurs
    /// during the request.
    pub async fn get_by_ids_strict(client: &Client, ids: Vec<u64>) -> Result<Vec<Self>> {
        GetFandomsRequest::new(ids)
            .send_request(client)
            .await?
            .try_into()
    }

    /// Blocks this fandom, hiding all its publications from your feed.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn block(&self, client: &Client) -> Result<&Self> {
        BlockFandomRequest::new(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Unblocks this fandom, making its publications visible in your feed again.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn unblock(&self, client: &Client) -> Result<&Self> {
        UnblockFandomRequest::new(self.id)
            .send_request(client)
            .await?;
        Ok(self)
    }

    /// Checks if this fandom is currently blocked by you.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    pub async fn check_blocked(&self, client: &Client) -> Result<bool> {
        Ok(CheckFandomBlockedRequest::new(self.id)
            .send_request(client)
            .await?
            .into())
    }
}
