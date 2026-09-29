use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::publication::kind::chat_message::SendChatMessageError;
use crate::models::{ChatMessage, ChatTag, Publication};
use crate::requests::raw::{RawChatMessage, RawChatTag, RawPublication};
use crate::{Client, Result};

#[derive(Default, Debug)]
pub enum Content<'a> {
    #[default]
    Text,
    Images(Vec<&'a [u8]>),
    Gif(&'a [u8]),
    Sticker {
        id: u64,
    },
    Voice(&'a [u8]),
}

#[derive(Default, Debug)]
pub struct Options<'a> {
    pub content: Content<'a>,
    pub chat_tag: ChatTag,
    pub text: Option<&'a str>,
    pub reply_to_id: Option<u64>,
    pub answer_to_id: Option<u64>,
    pub use_new_formatting: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SendChatMessageRequest<'a> {
    #[serde(skip)]
    content: Content<'a>,
    #[serde(rename = "tag")]
    chat_tag: RawChatTag,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    sticker_id: u64,
    #[serde(rename = "parentMessageId")]
    reply_to_id: u64,
    #[serde(rename = "quoteMessageId")]
    answer_to_id: u64,
    #[serde(rename = "newFormatting")]
    use_new_formatting: bool,
}
impl<'a> SendChatMessageRequest<'a> {
    pub(crate) fn new(options: Options<'a>) -> Self {
        Self {
            chat_tag: options.chat_tag.into(),
            text: options.text,
            sticker_id: match options.content {
                Content::Sticker { id } => id,
                _ => 0,
            },
            reply_to_id: options.reply_to_id.unwrap_or(0),
            answer_to_id: options.answer_to_id.unwrap_or(0),
            use_new_formatting: options.use_new_formatting,
            content: options.content,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    message: RawPublication<RawChatMessage>,
}

impl Request<Publication<ChatMessage>> for SendChatMessageRequest<'_> {
    type Response = Response;
    type Error = SendChatMessageError;

    async fn send(&self, client: &Client) -> Result<Publication<ChatMessage>> {
        let attachments: Vec<&[u8]> = match self.content {
            Content::Text | Content::Sticker { .. } => {
                vec![&[], &[]]
            }
            Content::Images(ref images) => {
                let mut attachments: Vec<&[u8]> = vec![&[], &[]];
                attachments.extend(images.iter());
                attachments
            }
            Content::Gif(gif) => vec![gif, &[]],
            Content::Voice(voice) => vec![&[], voice],
        };

        client
            .send_request("RChatMessageCreate", self, attachments)
            .await?
            .message
            .try_into()
    }
}
