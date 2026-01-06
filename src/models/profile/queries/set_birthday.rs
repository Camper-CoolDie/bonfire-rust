use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;

use crate::models::raw::RawMe;
use crate::models::Me;
use crate::{Client, Result};

#[derive(Deserialize)]
struct Response {
    #[serde(rename = "setBirthday")]
    me: RawMe,
}

impl Me {
    pub(crate) async fn _set_birthday(client: &Client, birthday: NaiveDate) -> Result<Me> {
        Ok(client
            .send_query::<_, Response>(
                "SetBirthdayMutation",
                include_str!("graphql/SetBirthdayMutation.graphql"),
                json!({ "birthday": birthday }),
            )
            .await?
            .me
            .into())
    }
}
