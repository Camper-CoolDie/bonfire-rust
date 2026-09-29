use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Auth;
use crate::models::auth::LoginError;
use crate::queries::raw::RawAuth;
use crate::queries::raw::auth::RawTfaRequired;
use crate::{Client, Error, Result};

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

#[derive(Deserialize)]
#[serde(tag = "__typename")]
pub(crate) enum LoginResult {
    #[serde(rename = "LoginResultSuccess")]
    Success(RawAuth),
    #[serde(rename = "LoginResultTfaRequired")]
    TfaRequired(RawTfaRequired),
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "loginEmail")]
    result: LoginResult,
}

impl Request<Auth> for LoginEmailQuery<'_> {
    type Response = Response;
    type Error = LoginError;

    async fn send(&self, client: &Client) -> Result<Auth> {
        let result = client
            .send_query_authless("LoginEmail", "auth/LoginEmail.graphql", self)
            .await?
            .result;

        match result {
            LoginResult::Success(success) => Ok(success.into()),
            LoginResult::TfaRequired(error) => Err(Error::RequestError(Box::new(
                LoginError::TfaRequired(error.into()),
            ))),
        }
    }
}
