mod error;
mod me;

use chrono::NaiveDate;
pub use error::{Error, TfaKind, TfaRequired};
pub use me::Me;
use serde::{Deserialize, Serialize};
use serde_json::json;

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
                include_str!("../../graphql/auth/LoginEmailMutation.graphql"),
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
                include_str!("../../graphql/auth/LogoutMutation.graphql"),
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
                include_str!("../../graphql/auth/MeQuery.graphql"),
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
                include_str!("../../graphql/auth/SetBirthdayMutation.graphql"),
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
                    include_str!("../../graphql/auth/LoginRefreshMutation.graphql"),
                    json!({ "refreshToken": auth.refresh_token }),
                )
                .await?
                .auth,
        );
        Ok(())
    }
}
