use serde::Deserialize;
use serde_json::json;

use crate::models::auth::Error;
use crate::models::Auth;
use crate::raw::auth::RawTfaRequired;
use crate::raw::RawAuth;
use crate::{Client, Query, Result};

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

pub(crate) struct LoginEmailQuery<'a> {
    email: &'a str,
    password: &'a str,
}
impl<'a> LoginEmailQuery<'a> {
    pub(crate) fn new(email: &'a str, password: &'a str) -> Self {
        Self { email, password }
    }
}

impl Query for LoginEmailQuery<'_> {
    type Target = Auth;

    async fn send_query(&self, client: &Client) -> Result<Auth> {
        match client
            .send_query::<_, Response>(
                "LoginEmailMutation",
                include_str!("graphql/LoginEmailMutation.graphql"),
                json!({
                    "input": {
                        "email": self.email,
                        "password": self.password,
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
