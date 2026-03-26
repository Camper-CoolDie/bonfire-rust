use std::time::Duration;

use serde::Serialize;

use crate::client::{EmptyResponse, InfallibleRequest, Request};
use crate::models::ChatTag;
use crate::requests::raw::RawChatTag;
use crate::{Client, Result, RootError};

#[derive(Serialize)]
pub(crate) struct TypingRequest {
    tag: RawChatTag,
}
impl TypingRequest {
    pub(crate) const PERIOD: Duration = Duration::from_secs(5);

    pub(crate) fn new(tag: ChatTag) -> Self {
        Self { tag: tag.into() }
    }
}

impl Request for TypingRequest {
    type Response = EmptyResponse;
    type Error = InfallibleRequest<RootError>;

    async fn send_request(&self, client: &Client) -> Result<EmptyResponse> {
        client.send_request("RChatTyping", self, Vec::new()).await
    }
}
