use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Chat, ChatTag};
use crate::requests::raw::{RawChat, RawChatTag};
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    chat: RawChat,
}

impl TryFrom<Response> for Chat {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.chat.try_into()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) enum GetChatRequest {
    Tag(RawChatTag),
    MessageId(u64),
}
impl GetChatRequest {
    pub(crate) fn new_by_tag(tag: ChatTag) -> Self {
        Self::Tag(tag.into())
    }

    pub(crate) fn new_by_message_id(id: u64) -> Self {
        Self::MessageId(id)
    }
}

impl Request for GetChatRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client.send_request("RChatGet", self, Vec::new()).await
    }
}
