use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Fandom;
use crate::requests::raw::RawFandom;
use crate::{Client, Error, Result, RootError, UnavailableError};

#[derive(Deserialize)]
pub(crate) struct Response {
    fandoms: Vec<RawFandom>,
    #[serde(skip)]
    ids: Vec<u64>,
}
impl Response {
    fn enumerate_fandoms(fandoms: Vec<RawFandom>) -> HashMap<u64, RawFandom> {
        fandoms
            .into_iter()
            .map(|fandom| (fandom.id, fandom))
            .collect()
    }
}

// Will return the last element of Response::fandoms or NotFound error if there's none
impl TryFrom<Response> for Fandom {
    type Error = Error;

    fn try_from(mut value: Response) -> Result<Self> {
        value
            .fandoms
            .pop()
            .ok_or(Error::RootError(RootError::Unavailable(
                UnavailableError::NotFound,
            )))?
            .try_into()
    }
}

impl TryFrom<Response> for Vec<Fandom> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        let mut map = Response::enumerate_fandoms(value.fandoms);

        // The server returns an unsorted list of fandoms, we sort it by `self.ids` and return the
        // NotFound error if there's at least one missing fandom
        value
            .ids
            .into_iter()
            .map(|id| {
                map.remove(&id)
                    .ok_or(Error::RootError(RootError::Unavailable(
                        UnavailableError::NotFound,
                    )))
                    .and_then(Fandom::try_from)
            })
            .collect()
    }
}

impl TryFrom<Response> for Vec<Option<Fandom>> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        let mut map = Response::enumerate_fandoms(value.fandoms);

        // The server returns an unsorted list of fandoms, we sort it by `self.ids` and return
        // `None` for every missing fandom
        value
            .ids
            .into_iter()
            .map(|id| map.remove(&id).map(Fandom::try_from).transpose())
            .collect()
    }
}

#[derive(Serialize)]
pub(crate) struct GetFandomsRequest {
    #[serde(rename = "fandomsIds")]
    ids: Vec<u64>,
}
impl GetFandomsRequest {
    pub(crate) fn new(ids: Vec<u64>) -> Self {
        Self { ids }
    }

    pub(crate) fn new_single(id: u64) -> Self {
        Self { ids: vec![id] }
    }
}

impl Request for GetFandomsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        let mut response = client
            .send_request("RFandomsGetAllById", self, Vec::new())
            .await?;

        response.ids.clone_from(&self.ids);
        Ok(response)
    }
}
