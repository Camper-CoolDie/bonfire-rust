use serde_json::json;

use crate::models::account::Info;
use crate::raw::account::RawInfo;
use crate::{Client, Request, Result};

pub(crate) struct GetInfoRequest<'a> {
    id: Option<i64>,
    name: Option<&'a str>,
}
impl<'a> GetInfoRequest<'a> {
    pub(crate) fn new_by_id(id: i64) -> Self {
        Self {
            id: Some(id),
            name: None,
        }
    }

    pub(crate) fn new_by_name(name: &'a str) -> Self {
        Self {
            id: None,
            name: Some(name),
        }
    }
}

impl Request for GetInfoRequest<'_> {
    type Target = Info;

    async fn send_request(&self, client: &Client) -> Result<Info> {
        client
            .send_request::<_, RawInfo>(
                "RAccountsGetProfile",
                json!({
                    "accountId": self.id,
                    "accountName": self.name,
                }),
                Vec::default(),
            )
            .await?
            .try_into()
    }
}
