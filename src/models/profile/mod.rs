mod queries;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::Auth;
use crate::{Client, Result};

/// Represents information about an authenticated user.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Me {
    /// A unique identifier of your account. Isn't guaranteed to be an integer
    pub id: String,
    /// Your name
    #[serde(rename = "username")]
    pub name: String,
    /// Your email
    pub email: String,
    #[serde(
        serialize_with = "crate::models::serialize_level",
        deserialize_with = "crate::models::deserialize_level"
    )]
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
    /// use bonfire::models::Me;
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     // ...
    ///     let info = Me::get(&mut client).await.unwrap();
    ///     println!("logged in as {}", info.name);
    /// }
    /// ```
    pub async fn get(client: &mut Client) -> Result<Self> {
        Auth::me(client).await
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
    /// use bonfire::models::Me;
    /// use bonfire::Client;
    /// use chrono::NaiveDate;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     // ...
    ///     let birthday = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    ///     Me::set_birthday(&mut client, birthday).await.unwrap();
    /// }
    /// ```
    pub async fn set_birthday(client: &mut Client, birthday: NaiveDate) -> Result<Me> {
        Me::_set_birthday(client, birthday).await
    }
}
