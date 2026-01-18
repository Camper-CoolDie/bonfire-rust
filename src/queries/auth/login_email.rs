use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::auth::Error;
use crate::models::Auth;
use crate::queries::raw::auth::RawTfaRequired;
use crate::queries::raw::RawAuth;
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

#[derive(Serialize)]
struct LoginInput<'a> {
    email: &'a str,
    password: &'a str,
}

#[derive(Serialize)]
pub(crate) struct LoginEmailQuery<'a> {
    input: LoginInput<'a>,
}
impl<'a> LoginEmailQuery<'a> {
    pub(crate) fn new(email: &'a str, password: &'a str) -> Self {
        Self {
            input: LoginInput { email, password },
        }
    }
}

impl Request for LoginEmailQuery<'_> {
    type Target = Auth;

    async fn send_request(&self, client: &Client) -> Result<Auth> {
        match client
            .send_query::<_, Response>(
                "LoginEmailMutation",
                include_str!("graphql/LoginEmailMutation.graphql"),
                self,
            )
            .await?
            .result
        {
            LoginResult::Success(success) => Ok(success.into()),
            LoginResult::TfaRequired(error) => Err(Error::TfaRequired(error.into()).into()),
        }
    }
}
