use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Profile;
use crate::queries::raw::RawProfile;
use crate::{Client, MeliorError, Result};

#[derive(Serialize)]
pub(crate) struct GetProfileQuery {}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "me")]
    profile: RawProfile,
}

impl Request<Profile> for GetProfileQuery {
    type Response = Response;
    type Error = InfallibleRequest<MeliorError>;

    async fn send(&self, client: &Client) -> Result<Profile> {
        client
            .send_query("MeQuery", "auth/MeQuery.graphql", self)
            .await?
            .profile
            .try_into()
    }
}
