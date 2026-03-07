use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Profile;
use crate::queries::raw::RawProfile;
use crate::{Client, MeliorError, Result};

#[derive(Deserialize)]
pub(crate) struct Response {
    me: RawProfile,
}

impl From<Response> for Profile {
    fn from(value: Response) -> Self {
        value.me.into()
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
            .send_query("MeQuery", include_str!("graphql/MeQuery.graphql"), self)
            .await
    }
}
