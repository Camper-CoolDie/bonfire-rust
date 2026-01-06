use serde_json::json;

use crate::models::account::Info;
use crate::models::raw::account::RawInfo;
use crate::models::Account;
use crate::{Client, Result};

impl Account {
    pub(crate) async fn _get_info(
        client: &Client,
        id: Option<i64>,
        name: Option<&str>,
    ) -> Result<Info> {
        client
            .send_request::<_, RawInfo>(
                "RAccountsGetProfile",
                json!({
                    "accountId": id,
                    "accountName": name
                }),
                Vec::default(),
            )
            .await?
            .try_into()
    }
}
