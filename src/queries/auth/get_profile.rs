use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Profile;
use crate::queries::raw::RawProfile;
use crate::{Client, Error, MeliorError, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    profile: RawProfile,
}

impl TryFrom<Response> for Profile {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.profile.try_into()
    }
}

#[derive(Serialize)]
pub(crate) struct GetProfileQuery {}
impl GetProfileQuery {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Request for GetProfileQuery {
    type Response = Response;
    type Error = InfallibleRequest<MeliorError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_query("MeQuery", "auth/MeQuery.graphql", self)
            .await
    }
}
