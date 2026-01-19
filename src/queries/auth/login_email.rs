use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::{Auth, AuthError};
use crate::queries::raw::auth::RawTfaRequired;
use crate::queries::raw::RawAuth;
use crate::{Client, Error, Result};

#[derive(Deserialize)]
#[serde(tag = "__typename")]
enum LoginResult {
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

impl TryFrom<Response> for Auth {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        match value.result {
            LoginResult::Success(success) => Ok(success.into()),
            LoginResult::TfaRequired(error) => Err(AuthError::TfaRequired(error.into()).into()),
        }
    }
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
    type Response = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query(
                "LoginEmailMutation",
                include_str!("graphql/LoginEmailMutation.graphql"),
                self,
            )
            .await
    }
}
