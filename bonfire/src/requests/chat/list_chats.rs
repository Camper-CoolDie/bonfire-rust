use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::Chat;
use crate::requests::raw::RawChat;
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "units")]
    chats: Vec<RawChat>,
}

impl TryFrom<Response> for Vec<Chat> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.chats.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
pub(crate) struct ListChatsRequest {
    offset: usize,
}
impl ListChatsRequest {
    pub(crate) const PAGE_SIZE: usize = 10;

    pub(crate) fn new(offset: usize) -> Self {
        Self { offset }
    }
}

impl Request for ListChatsRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client.send_request("RChatsGetAll", self, Vec::new()).await
    }
}
