use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::models::Me;
use crate::queries::raw::RawMe;
use crate::{Client, Query, Result};

#[derive(Deserialize)]
struct Response {
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

impl Query for SetBirthdayQuery {
    type Target = Me;

    async fn send_query(&self, client: &Client) -> Result<Me> {
        Ok(client
            .send_query::<_, Response>(
                "SetBirthdayMutation",
                include_str!("graphql/SetBirthdayMutation.graphql"),
                self,
            )
            .await?
            .me
            .into())
    }
}
