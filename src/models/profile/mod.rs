use chrono::NaiveDate;

use crate::client::Request as _;
use crate::queries::auth::MeQuery;
use crate::queries::profile::SetBirthdayQuery;
use crate::{Client, Result};

/// Represents detailed information about the currently authenticated user's profile.
#[derive(Default, Clone, Debug)]
pub struct Me {
    /// The unique identifier of your account (not guaranteed to be an integer)
    pub id: String,
    /// Your display name
    pub name: String,
    /// Your registered email address
    pub email: String,
    /// Your last known level, cached by the API
    pub cached_level: f64,
    /// Your date of birth, or `None` if not set
    pub birthday: Option<NaiveDate>,
    /// Indicates whether you are allowed to view NSFW content, or `None` if `birthday` is also
    /// `None`
    pub is_nsfw_allowed: Option<bool>,
}
impl Me {
    /// Retrieves information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Me;
    /// # use bonfire::{Client, Result};
    /// #
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let me = Me::get(client).await?;
    /// println!("Logged in as {}", me.name);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get(client: &Client) -> Result<Self> {
        Ok(MeQuery::new().send_request(client).await?.into())
    }

    /// Sets the authenticated user's birthday.
    ///
    /// # Errors
    ///
    /// Returns [`Error`][crate::Error] if an error occurs while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Me;
    /// # use bonfire::{Client, Result};
    /// use chrono::NaiveDate;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// # let client = &Client::default();
    /// let birthday = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    /// Me::set_birthday(client, birthday).await?;
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn set_birthday(client: &Client, birthday: NaiveDate) -> Result<Me> {
        Ok(SetBirthdayQuery::new(birthday)
            .send_request(client)
            .await?
            .into())
    }
}
