use chrono::NaiveDate;

use crate::client::Request;
use crate::queries::auth::MeQuery;
use crate::queries::profile::SetBirthdayQuery;
use crate::{Client, Result};

/// Represents information about an authenticated user.
#[derive(Default, Clone, Debug)]
pub struct Me {
    /// A unique identifier of your account. Isn't guaranteed to be an integer
    pub id: String,
    /// Your name
    pub name: String,
    /// Your email
    pub email: String,
    /// Your cached level
    pub cached_level: f32,
    /// Your day of birth or `None` if unset
    pub birthday: Option<NaiveDate>,
    /// Are you allowed to see NSFW posts? None if `birthday` is `None`
    pub is_nsfw_allowed: Option<bool>,
}
impl Me {
    /// Get information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use bonfire::models::Me;
    /// # use bonfire::Result;
    /// use bonfire::Client;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<()> {
    /// let client = Client::default();
    /// client.login("email", "password").await?;
    ///
    /// let me = Me::get(&client).await.unwrap();
    /// println!("Logged in as {}", me.name);
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn get(client: &Client) -> Result<Self> {
        MeQuery::new()
            .send_request(client)
            .await
            .map(|r| r.me.into())
    }

    /// Set your birthday.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
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
    /// # let client = Client::default();
    /// let birthday = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    /// Me::set_birthday(&client, birthday).await.unwrap();
    /// #    Ok(())
    /// # }
    /// ```
    pub async fn set_birthday(client: &Client, birthday: NaiveDate) -> Result<Me> {
        SetBirthdayQuery::new(birthday)
            .send_request(client)
            .await
            .map(|r| r.me.into())
    }
}
