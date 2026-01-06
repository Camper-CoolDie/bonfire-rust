use serde::Deserialize;
use serde_json::json;

use crate::models::auth::Error;
use crate::models::raw::auth::RawTfaRequired;
use crate::models::raw::RawAuth;
use crate::models::Auth;
use crate::{Client, Result};

#[derive(Deserialize)]
#[serde(tag = "__typename")]
enum LoginResult {
    #[serde(rename = "LoginResultSuccess")]
    Success(RawAuth),
    #[serde(rename = "LoginResultTfaRequired")]
    TfaRequired(RawTfaRequired),
}

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "loginEmail")]
    result: LoginResult,
}

impl Auth {
    pub(crate) async fn _login_email(client: &Client, email: &str, password: &str) -> Result<Self> {
        match client
            .send_query::<_, Response>(
                "LoginEmailMutation",
                include_str!("graphql/LoginEmailMutation.graphql"),
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
            LoginResult::Success(success) => Ok(success.into()),
            LoginResult::TfaRequired(error) => Err(Error::TfaRequired(error.into()).into()),
        }
    }
}
