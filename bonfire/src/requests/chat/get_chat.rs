use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Chat, ChatTag};
use crate::requests::raw::{RawChat, RawChatTag};
use crate::{Client, Result, RootError};

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

#[derive(Deserialize)]
pub(crate) struct Response {
    chat: RawChat,
}

impl Request<Chat> for GetChatRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Chat> {
        client
            .send_request("RChatGet", self, Vec::new())
            .await?
            .chat
            .try_into()
    }
}
