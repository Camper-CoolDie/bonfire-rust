use std::fmt;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::{Client, Result};

/// Represents an authentication session.
#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    /// The session's access token
    pub access_token: String,
    /// The session's refresh token
    pub refresh_token: String,
}
impl Auth {
    /// Log in using email.
    ///
    /// # Errors
    ///
    /// Returns [Error::TfaRequired] if a TFA is required to continue logging in or
    /// [Error][crate::Error] if an error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bonfire::models::auth::Me;
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     Auth::login(&mut client, "email", "password").await.unwrap();
    /// }
    /// ```
    pub async fn login(client: &mut Client, email: &str, password: &str) -> Result<()> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "loginEmail")]
            result: LoginResult,
        }

        #[derive(Deserialize)]
        #[serde(tag = "__typename")]
        enum LoginResult {
            #[serde(rename = "LoginResultSuccess")]
            Success(Auth),
            #[serde(rename = "LoginResultTfaRequired")]
            TfaRequired(TfaRequired),
        }

        match client
            .send_query::<_, Response>(
                "LoginEmailMutation",
                include_str!("../graphql/auth/LoginEmailMutation.graphql"),
                json!({
                    "input": {
                        "email": email,
                        "password": password,
                    }
                }),
            )
            .await?
            .result
        {
            LoginResult::Success(success) => {
                client.auth = Some(success);
                Ok(())
            }
            LoginResult::TfaRequired(error) => Err(Error::TfaRequired(error).into()),
        }
    }

    /// Log out. The client becomes unauthenticated.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bonfire::models::auth::Me;
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     // ...
    ///     Auth::logout(&mut client).await.unwrap();
    /// }
    /// ```
    pub async fn logout(client: &mut Client) -> Result<()> {
        #[derive(Deserialize)]
        struct Response {}

        client
            .send_query::<_, Response>(
                "LogoutMutation",
                include_str!("../graphql/auth/LogoutMutation.graphql"),
                json!({}),
            )
            .await?;
        client.auth = None;
        Ok(())
    }

    /// Get information about the currently authenticated user.
    ///
    /// # Errors
    ///
    /// Returns [Error][crate::Error] if an error occurred while sending the request.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use bonfire::models::auth::Me;
    /// use bonfire::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     // ...
    ///     let me = Auth::me(&mut client).await.unwrap();
    ///     println!("logged in as {}", me.name);
    /// }
    /// ```
    pub async fn me(client: &mut Client) -> Result<Me> {
        #[derive(Deserialize)]
        struct Response {
            me: Me,
        }

        Ok(client
            .send_query::<_, Response>(
                "MeQuery",
                include_str!("../graphql/auth/MeQuery.graphql"),
                json!({}),
            )
            .await?
            .me)
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
    /// use bonfire::models::auth::Me;
    /// use bonfire::Client;
    /// use chrono::NaiveDate;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = Client::connect().await.unwrap();
    ///     // ...
    ///     let birthday = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    ///     Auth::set_birthday(&mut client, birthday).await.unwrap();
    /// }
    /// ```
    pub async fn set_birthday(client: &mut Client, birthday: NaiveDate) -> Result<Me> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "setBirthday")]
            me: Me,
        }

        Ok(client
            .send_query::<_, Response>(
                "SetBirthdayMutation",
                include_str!("../graphql/auth/SetBirthdayMutation.graphql"),
                json!({ "birthday": birthday }),
            )
            .await?
            .me)
    }

    pub(crate) async fn refresh(client: &mut Client) -> Result<()> {
        #[derive(Deserialize)]
        struct Response {
            #[serde(rename = "loginRefresh")]
            auth: Auth,
        }

        let auth = client.auth.clone().ok_or(Error::Unauthenticated)?;
        client.auth = Some(
            client
                .send_query_without_auth::<_, Response>(
                    "LoginRefreshMutation",
                    include_str!("../graphql/auth/LoginRefreshMutation.graphql"),
                    json!({ "refreshToken": auth.refresh_token }),
                )
                .await?
                .auth,
        );
        Ok(())
    }
}

/// Represents errors that can occur while authenticating.
///
/// # Source
///
/// An `auth::Error` can be the result of a non-standart response or an unauthenticated client.
#[derive(Error, Debug)]
pub enum Error {
    /// TFA is required to continue logging in
    #[error("TFA is required to continue logging in ({0})")]
    TfaRequired(TfaRequired),
    /// Client is unauthenticated
    #[error("unauthenticated client")]
    Unauthenticated,
}

/// Represents data to continue logging in using TFA (Two-Factor Authentication).
#[derive(Clone, Debug, Deserialize)]
pub struct TfaRequired {
    /// A type of the TFA
    #[serde(rename = "tfaType")]
    pub kind: TfaKind,
    /// A wait token of the TFA
    #[serde(rename = "tfaWaitToken")]
    pub wait_token: String,
}

impl fmt::Display for TfaRequired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

/// Represents a type of the TFA.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TfaKind {
    /// Log in again using a TOTP (Time-based One Time Password)
    Totp,
    /// Log in after an owner of the account verified the login through a link sent to their email
    EmailLink,
}

impl fmt::Display for TfaKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Totp => write!(f, "TOTP"),
            Self::EmailLink => write!(f, "email link"),
        }
    }
}

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
    /// use bonfire::models::auth::Me;
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
    #[inline]
    pub async fn get(client: &mut Client) -> Result<Self> {
        Auth::me(client).await
    }
}
