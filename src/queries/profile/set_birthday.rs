use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::Me;
use crate::queries::raw::RawMe;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "setBirthday")]
    me: RawMe,
}

#[derive(Serialize)]
pub(crate) struct SetBirthdayQuery {
    birthday: NaiveDate,
}
impl SetBirthdayQuery {
    pub(crate) fn new(birthday: NaiveDate) -> Self {
        Self { birthday }
    }
}

impl Request for SetBirthdayQuery {
    type Response = Response;
    type Target = Me;

    async fn send_request(&self, client: &Client) -> Result<Me> {
        Ok(client
            .send_query(
                "SetBirthdayMutation",
                include_str!("graphql/SetBirthdayMutation.graphql"),
                self,
            )
            .await?
            .me
            .into())
    }
}
