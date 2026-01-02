use serde::Deserialize;
use serde_json::json;

use crate::models::auth::Error;
use crate::models::raw::RawAuth;
use crate::models::Auth;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "loginRefresh")]
    auth: RawAuth,
}

impl Auth {
    pub(crate) async fn _login_refresh(client: &mut Client) -> Result<()> {
        let auth = client.auth.clone().ok_or(Error::Unauthenticated)?;

        client.auth = Some(
            client
                .send_query_without_auth::<_, Response>(
                    "LoginRefreshMutation",
                    include_str!("graphql/LoginRefreshMutation.graphql"),
                    json!({ "refreshToken": auth.refresh_token }),
                )
                .await?
                .auth
                .into(),
        );
        Ok(())
    }
}
