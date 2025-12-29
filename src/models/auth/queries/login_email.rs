use serde::Deserialize;
use serde_json::json;

use crate::models::auth::{Error, TfaRequired};
use crate::models::Auth;
use crate::{Client, Result};

#[derive(Deserialize)]
#[serde(tag = "__typename")]
enum LoginResult {
    #[serde(rename = "LoginResultSuccess")]
    Success(Auth),
    #[serde(rename = "LoginResultTfaRequired")]
    TfaRequired(TfaRequired),
}

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "loginEmail")]
    result: LoginResult,
}

impl Auth {
    pub(crate) async fn _login_email(
        client: &mut Client,
        email: &str,
        password: &str,
    ) -> Result<()> {
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
            LoginResult::Success(success) => {
                client.auth = Some(success);
                Ok(())
            }
            LoginResult::TfaRequired(error) => Err(Error::TfaRequired(error).into()),
        }
    }
}
