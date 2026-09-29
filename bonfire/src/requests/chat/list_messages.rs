use std::num::NonZeroUsize;

use chrono::{DateTime, Utc};
use futures::Stream;
use nonzero_ext::nonzero;
use serde::{Deserialize, Serialize};

use crate::client::{InfallibleRequest, PaginatedRequest, Request};
use crate::models::{ChatMessage, ChatTag, Publication};
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Client, Error, Result, RootError, UnavailableError};

// 51 if message_id points to a valid message (25 previous, target, 25 next)
const PAGE_SIZE: NonZeroUsize = nonzero!(50usize);

#[derive(Default, Debug)]
pub enum Anchor {
    #[default]
    Newest,
    Message {
        id: u64,
    },
    Offset {
        date: DateTime<Utc>,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ListMessagesRequest {
    #[serde(rename = "tag")]
    chat_tag: RawChatTag,
    #[serde(rename = "old")]
    newest_first: bool,
    offset_date: i64,
    message_id: u64,
}
impl ListMessagesRequest {
    pub(crate) fn new(chat_tag: ChatTag, newest_first: bool, anchor: &Anchor) -> Self {
        let message_id = match anchor {
            Anchor::Message { id } => *id,
            _ => 0,
        };
        let offset_date = match anchor {
            Anchor::Offset { date } => date.timestamp_millis(),
            _ => 0,
        };

        Self {
            chat_tag: chat_tag.into(),
            newest_first: offset_date == 0 && message_id == 0 || newest_first,
            message_id,
            offset_date,
        }
    }

    pub(crate) fn stream(
        self,
        client: &Client,
    ) -> impl Stream<Item = Result<Publication<ChatMessage>>> {
        PaginatedRequest::new(
            client,
            self,
            |request, messages| {
                if let Some(message) = messages.last() {
                    request.offset_date = message.created_at.timestamp_millis();
                    // Continue streaming messages further, if message_id was set
                    request.message_id = 0;
                }
            },
            PAGE_SIZE,
        )
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "units")]
    messages: Vec<RawPublication<RawChatMessage>>,
}

impl Request<Vec<Publication<ChatMessage>>> for ListMessagesRequest {
    type Response = Response;
    type Error = InfallibleRequest<RootError>;

    async fn send(&self, client: &Client) -> Result<Vec<Publication<ChatMessage>>> {
        let messages = client
            .send_request("RChatMessageGetAll", self, Vec::new())
            .await?
            .messages;

        // No message was found
        if self.message_id != 0 && messages.len() <= PAGE_SIZE.get() {
            return Err(Error::RootError(RootError::Unavailable(
                UnavailableError::NotFound,
            )));
        }

        // Received list is always sorted from oldest to newest
        let iter = messages.into_iter().map(TryInto::try_into);
        if self.newest_first {
            iter.rev().collect()
        } else {
            iter.collect()
        }
    }
}
