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
        let mut order: HashMap<u64, RawFandom> = value
            .fandoms
            .into_iter()
            .map(|fandom| (fandom.id, fandom))
            .collect();

        // Look up each requested id, preserving order
        let mut result = Vec::with_capacity(value.ids.len());
        for &id in &value.ids {
            let fandom = order
                .remove(&id)
                .ok_or(Error::RootError(RootError::Unavailable(
                    UnavailableError::NotFound,
                )))?;
            result.push(Fandom::try_from(fandom)?);
        }

        Ok(result)
    }
}

impl TryFrom<Response> for Vec<Option<Fandom>> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        let mut order: HashMap<u64, RawFandom> = value
            .fandoms
            .into_iter()
            .map(|fandom| (fandom.id, fandom))
            .collect();

        // Look up each requested id, preserving order
        value
            .ids
            .iter()
            .map(|&id| order.remove(&id).map(Fandom::try_from).transpose())
            .collect()
    }
}

#[derive(Serialize)]
pub(crate) struct ListFandomsRequest<'a> {
    #[serde(rename = "fandomsIds")]
    ids: &'a [u64],
}
impl<'a> ListFandomsRequest<'a> {
    pub(crate) fn new(ids: &'a [u64]) -> Self {
        Self { ids }
    }
}

impl Request for ListFandomsRequest<'_> {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        let mut response = client
            .send_request("RFandomsGetAllById", self, Vec::new())
            .await?;

        response.ids = self.ids.to_vec();
        Ok(response)
    }
}
