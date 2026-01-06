use serde::Deserialize;
use serde_json::json;

use crate::models::raw::RawAuth;
use crate::models::Auth;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "loginRefresh")]
    auth: RawAuth,
}

impl Auth {
    pub(crate) async fn _login_refresh(&self, client: &Client) -> Result<Self> {
        Ok(client
            .send_refresh_query::<_, Response>(
                "LoginRefreshMutation",
                include_str!("graphql/LoginRefreshMutation.graphql"),
                json!({ "refreshToken": self.refresh_token }),
            )
            .await?
            .auth
            .into())
    }
}
