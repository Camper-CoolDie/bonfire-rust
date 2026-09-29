use std::collections::HashMap;
use std::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Fandom;
use crate::requests::raw::RawFandom;
use crate::{Client, Error, Result, RootError, UnavailableError};

const REQUEST_NAME: &str = "RFandomsGetAllById";

#[derive(Serialize)]
pub(crate) struct ListFandomsRequest<'a, T> {
    #[serde(rename = "fandomsIds")]
    ids: &'a [u64],
    #[serde(skip)]
    _marker: PhantomData<T>,
}
impl<'a, T> ListFandomsRequest<'a, T> {
    fn new(ids: &'a [u64]) -> Self {
        Self {
            ids,
            _marker: PhantomData,
        }
    }
}

impl<'a> ListFandomsRequest<'a, Fandom> {
    // Pass &[u64; 1] instead of u64 so borrowing is handled by the caller
    pub(crate) fn new_by_id(id: &'a [u64; 1]) -> Self {
        Self::new(id)
    }
}

impl<'a> ListFandomsRequest<'a, Vec<Option<Fandom>>> {
    pub(crate) fn new_by_ids(ids: &'a [u64]) -> Self {
        Self::new(ids)
    }
}

impl<'a> ListFandomsRequest<'a, Vec<Fandom>> {
    pub(crate) fn new_by_ids_strict(ids: &'a [u64]) -> Self {
        Self::new(ids)
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    fandoms: Vec<RawFandom>,
}

impl Request<Fandom> for ListFandomsRequest<'_, Fandom> {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    // Will return the last element of Response::fandoms or NotFound error if there's none
    async fn send(&self, client: &Client) -> Result<Fandom> {
        client
            .send_request(REQUEST_NAME, self, Vec::new())
            .await?
            .fandoms
            .pop()
            .ok_or(Error::RootError(RootError::Unavailable(
                UnavailableError::NotFound,
            )))?
            .try_into()
    }
}

impl Request<Vec<Fandom>> for ListFandomsRequest<'_, Vec<Fandom>> {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Fandom>> {
        let response = client.send_request(REQUEST_NAME, self, Vec::new()).await?;

        let mut order: HashMap<u64, RawFandom> = response
            .fandoms
            .into_iter()
            .map(|fandom| (fandom.id, fandom))
            .collect();

        // Look up each requested id, preserving order
        let mut target = Vec::with_capacity(self.ids.len());
        for id in self.ids {
            let fandom = order
                .remove(id)
                .ok_or(Error::RootError(RootError::Unavailable(
                    UnavailableError::NotFound,
                )))?;
            target.push(Fandom::try_from(fandom)?);
        }

        Ok(target)
    }
}

impl Request<Vec<Option<Fandom>>> for ListFandomsRequest<'_, Vec<Option<Fandom>>> {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Option<Fandom>>> {
        let response = client.send_request(REQUEST_NAME, self, Vec::new()).await?;

        let mut order: HashMap<u64, RawFandom> = response
            .fandoms
            .into_iter()
            .map(|fandom| (fandom.id, fandom))
            .collect();

        // Look up each requested id, preserving order
        self.ids
            .iter()
            .map(|id| order.remove(id).map(Fandom::try_from).transpose())
            .collect()
    }
}
