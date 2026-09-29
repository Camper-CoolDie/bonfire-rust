use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Profile;
use crate::queries::raw::RawProfile;
use crate::{Client, MeliorError, Result};

#[derive(Serialize)]
pub(crate) struct SetBirthdayQuery {
    birthday: NaiveDate,
}
impl SetBirthdayQuery {
    pub(crate) fn new(birthday: NaiveDate) -> Self {
        Self { birthday }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "setBirthday")]
    profile: RawProfile,
}

impl Request<Profile> for SetBirthdayQuery {
    type Response = Response;
    type Error = InfallibleRequest<MeliorError>;

    async fn send(&self, client: &Client) -> Result<Profile> {
        client
            .send_query("SetBirthday", "profile/SetBirthday.graphql", self)
            .await?
            .profile
            .try_into()
    }
}
