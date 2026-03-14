use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Profile;
use crate::queries::raw::RawProfile;
use crate::{Client, Error, MeliorError, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "setBirthday")]
    profile: RawProfile,
}

impl TryFrom<Response> for Profile {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.profile.try_into()
    }
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
    type Error = InfallibleRequest<MeliorError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query(
                "SetBirthdayMutation",
                include_str!("graphql/SetBirthdayMutation.graphql"),
                self,
            )
            .await
    }
}
