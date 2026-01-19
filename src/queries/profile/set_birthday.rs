use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::queries::raw::RawMe;
use crate::{Client, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "setBirthday")]
    pub(crate) me: RawMe,
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
    type Target = Response;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query::<_, Response>(
                "SetBirthdayMutation",
                include_str!("graphql/SetBirthdayMutation.graphql"),
                self,
            )
            .await
    }
}
