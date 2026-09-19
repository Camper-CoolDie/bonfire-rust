use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, Request};
use crate::models::{Chat, ChatMessage, ChatTag, Publication};
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Client, Error, Result, RootError};

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "units")]
    messages: Vec<RawPublication<RawChatMessage>>,
}

impl TryFrom<Response> for Vec<Publication<ChatMessage>> {
    type Error = Error;

    fn try_from(value: Response) -> Result<Self> {
        value.messages.into_iter().map(TryInto::try_into).collect()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListMessagesRequest {
    #[serde(rename = "tag")]
    pub chat_tag: RawChatTag,
    pub offset_date: i64,
    #[serde(rename = "old")]
    pub newest_first: bool,
    pub message_id: u64,
}
impl ListMessagesRequest {
    // 51 if message_id points to a valid message (25 previous, target, 25 next)
    // WARN: If response contains messages from shadow-banned users they are omitted, so the page
    // size becomes less than 50. This is a server-side bug!
    pub(crate) const PAGE_SIZE: usize = 50;

    pub(crate) fn new(
        chat_tag: ChatTag,
        offset_date: Option<DateTime<Utc>>,
        newest_first: bool,
        message_id: Option<u64>,
    ) -> Self {
        Self {
            chat_tag: chat_tag.into(),
            offset_date: offset_date.map_or(0, |date| date.timestamp_millis()),
            newest_first,
            message_id: message_id.unwrap_or(0),
        }
    }
}

impl Request for ListMessagesRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<Response> {
        client
            .send_request("RChatMessageGetAll", self, Vec::new())
            .await
    }
}
