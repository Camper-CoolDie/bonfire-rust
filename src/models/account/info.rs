use std::ops::Range;
use std::result::Result as StdResult;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;

use crate::models::{EmptyResponse, Gender, ImageRef, Link, Post};
use crate::{Client, Result};

/// The number of links an account can contain.
pub const LINKS_COUNT: usize = 7;

/// The allowed range for an age.
pub const AGE_RANGE: Range<i64> = 0..101;

/// The maximum allowed status length.
pub const STATUS_MAX_LENGTH: usize = 100;

/// The maximum allowed description length.
pub const DESCRIPTION_MAX_LENGTH: usize = 1000;

#[derive(Deserialize, Serialize)]
struct Links {
    links: Vec<Link>,
}

/// Represents information about an account.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// The date when this account was created
    #[serde(
        rename = "dateCreate",
        serialize_with = "crate::models::serialize_timestamp_millis",
        deserialize_with = "crate::models::deserialize_timestamp_millis"
    )]
    pub created_at: DateTime<Utc>,
    /// The date when this account's ban ends
    #[serde(
        rename = "banDate",
        serialize_with = "crate::models::serialize_timestamp_millis_or_none",
        deserialize_with = "Info::deserialize_ban_timestamp"
    )]
    pub banned_until: Option<DateTime<Utc>>,
    /// The account's background
    #[serde(
        rename = "titleImage",
        serialize_with = "ImageRef::serialize_or_none",
        deserialize_with = "ImageRef::deserialize_or_none"
    )]
    pub background: Option<ImageRef>,
    /// The account's GIF background
    #[serde(
        rename = "titleImageGif",
        serialize_with = "ImageRef::serialize_or_none",
        deserialize_with = "ImageRef::deserialize_or_none"
    )]
    pub background_gif: Option<ImageRef>,
    /// Are you following this account?
    #[serde(rename = "isFollow")]
    pub is_following: bool,
    /// Is this account following you?
    #[serde(rename = "followsYou")]
    pub follows_me: bool,
    /// The number of users this account is followed to
    pub follows_count: i64,
    /// The number of users who are following this account
    pub followers_count: i64,
    /// The account's status
    #[serde(
        serialize_with = "crate::models::serialize_string_or_none",
        deserialize_with = "crate::models::deserialize_string_or_none"
    )]
    pub status: Option<String>,
    /// The account's age
    #[serde(
        serialize_with = "crate::models::serialize_i64_or_none",
        deserialize_with = "crate::models::deserialize_i64_or_none"
    )]
    pub age: Option<i64>,
    /// The account's description
    #[serde(
        serialize_with = "crate::models::serialize_string_or_none",
        deserialize_with = "crate::models::deserialize_string_or_none"
    )]
    pub description: Option<String>,
    /// The account's links
    #[serde(
        serialize_with = "Info::serialize_links",
        deserialize_with = "Info::deserialize_links"
    )]
    pub links: Vec<Link>,
    /// Your note to this account
    #[serde(
        serialize_with = "crate::models::serialize_string_or_none",
        deserialize_with = "crate::models::deserialize_string_or_none"
    )]
    pub note: Option<String>,
    /// A post which this account has pinned inside their profile
    pub pinned_post: Option<Post>,
    /// The account's bans count
    pub bans_count: i64,
    /// The account's warns count
    pub warns_count: i64,
    /// The account's total karma
    #[serde(
        serialize_with = "crate::models::serialize_level",
        deserialize_with = "crate::models::deserialize_level"
    )]
    pub karma_total: f32,
    /// The number of rates placed by this account
    #[serde(rename = "rates")]
    pub rates_count: i64,
    /// The sum of this account's positive rates (each rate's amount is rounded to 1)
    #[serde(rename = "ratesPositive")]
    pub positive_rates_sum: i64,
    /// The sum of this account's negative rates (each rate's amount is rounded to 1)
    #[serde(rename = "ratesNegative")]
    pub negative_rates_sum: i64,
    /// The number of fandoms this account can moderate
    #[serde(rename = "moderationFandomsCount")]
    pub moderating_fandoms_count: i64,
    /// The number of fandoms this account subscribed to
    #[serde(rename = "subscribedFandomsCount")]
    pub subscriptions_count: i64,
    /// The number of fandoms this account is viceroy in
    #[serde(rename = "viceroyFandomsCount")]
    pub viceroys_count: i64,
    /// The number of stickers this account has added to their collection
    pub stickers_count: i64,
    /// The number of users this account has blacklisted
    #[serde(rename = "blackAccountsCount")]
    pub blacklisted_accounts_count: i64,
    /// The number of fandoms this account has blacklisted
    #[serde(rename = "blackFandomsCount")]
    pub blacklisted_fandoms_count: i64,
}
impl Info {
    /// Get account information by its identifier.
    ///
    /// # Errors
    ///
    /// Returns [client::RootServerError::Unavailable][crate::client::RootServerError::Unavailable]
    /// if there's no account with the provided identifier or [Error][crate::Error] if any other
    /// error occurred while sending the request.
    pub async fn get_by_id(client: &mut Client, id: i64) -> Result<Self> {
        client
            .send_request::<_, Info>(
                "RAccountsGetProfile",
                json!({ "accountId": id }),
                Vec::default(),
            )
            .await
    }

    /// Get account information by its name.
    ///
    /// # Errors
    ///
    /// Returns [client::RootServerError::Unavailable][crate::client::RootServerError::Unavailable]
    /// if there's no account with the provided name or [Error][crate::Error] if any other error
    /// occurred while sending the request.
    pub async fn get_by_name(client: &mut Client, name: &str) -> Result<Self> {
        client
            .send_request::<_, Info>(
                "RAccountsGetProfile",
                json!({ "accountName": name }),
                Vec::default(),
            )
            .await
    }

    /// Set your age. Must be within [AGE_RANGE]. Zero or `None` means no age.
    ///
    /// # Errors
    ///
    /// Returns [client::RootServerError::Other][crate::client::RootServerError::Other] with the
    /// code `E_BAD_AGE` if the provided age is not within the range or [Error][crate::Error] if
    /// any other error occurred while sending the request.
    pub async fn set_age(client: &mut Client, age: Option<i64>) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetAge",
                json!({ "age": age.unwrap_or(0) }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }

    /// Set your status. Must be no longer than [STATUS_MAX_LENGTH]. Empty or `None` means no
    /// status.
    ///
    /// # Errors
    ///
    /// * [client::RootServerError::AccessDenied][crate::client::RootServerError::AccessDenied] if
    ///   you aren't yet allowed to change your status
    /// * [client::RootServerError::Other][crate::client::RootServerError::Other] with the code
    ///   `E_BAD_SIZE` if the provided status is longer than the maximum allowed length
    /// * [Error][crate::Error] if any other error occurred while sending the request.
    pub async fn set_status(client: &mut Client, status: Option<&str>) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsStatusSet",
                json!({ "status": status.unwrap_or("") }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }

    /// Set your description. Must be no longer than [DESCRIPTION_MAX_LENGTH]. Empty or `None`
    /// means no description.
    ///
    /// # Errors
    ///
    /// Returns [client::RootServerError::Other][crate::client::RootServerError::Other] with the
    /// code `E_BAD_SIZE` if the provided description is longer than the maximum allowed length or
    /// [Error][crate::Error] if any other error occurred while sending the request.
    pub async fn set_description(client: &mut Client, description: Option<&str>) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetDescription",
                json!({ "description": description.unwrap_or("") }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }

    /// Set your gender.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    pub async fn set_gender(client: &mut Client, gender: Gender) -> Result<()> {
        client
            .send_request::<_, EmptyResponse>(
                "RAccountsBioSetSex",
                json!({ "sex": gender }),
                Vec::default(),
            )
            .await?;
        Ok(())
    }

    fn serialize_links<S: Serializer>(value: &[Link], serializer: S) -> StdResult<S::Ok, S::Error> {
        let mut links = value.to_vec();
        links.resize(LINKS_COUNT, Link::default());

        Links { links }.serialize(serializer)
    }

    fn deserialize_links<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> StdResult<Vec<Link>, D::Error> {
        Ok(Links::deserialize(deserializer)?
            .links
            .into_iter()
            .filter(|link| !link.title.is_empty() && !link.uri.is_empty())
            .collect())
    }

    fn deserialize_ban_timestamp<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> StdResult<Option<DateTime<Utc>>, D::Error> {
        Ok(
            crate::models::deserialize_timestamp_millis_or_none(deserializer)?
                .filter(|date| *date > Utc::now()),
        )
    }
}
