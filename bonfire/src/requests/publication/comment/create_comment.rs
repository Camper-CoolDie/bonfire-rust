use serde::{Deserialize, Serialize};

use crate::client::Request;
use crate::models::publication::kind::comment::CreateCommentError;
use crate::models::{Comment, Publication};
use crate::requests::raw::{RawComment, RawPublication};
use crate::{Client, Result};

#[derive(Default, Debug)]
pub enum Content<'a> {
    #[default]
    Text,
    Images(Vec<&'a [u8]>),
    Gif {
        first_frame: &'a [u8],
        animated: &'a [u8],
    },
    Sticker {
        id: u64,
    },
}

#[derive(Default, Debug)]
pub struct Options<'a> {
    pub content: Content<'a>,
    pub parent_id: u64,
    pub text: Option<&'a str>,
    pub reply_to_id: Option<u64>,
    pub answer_to_id: Option<u64>,
    pub use_new_formatting: bool,
    pub watch_post: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateCommentRequest<'a> {
    #[serde(skip)]
    content: Content<'a>,
    #[serde(rename = "unitId")]
    parent_id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    sticker_id: u64,
    #[serde(rename = "parentCommentId")]
    reply_to_id: u64,
    #[serde(rename = "quoteId")]
    answer_to_id: u64,
    #[serde(rename = "newFormatting")]
    use_new_formatting: bool,
    watch_post: bool,
}
impl<'a> CreateCommentRequest<'a> {
    pub(crate) fn new(options: Options<'a>) -> Self {
        Self {
            parent_id: options.parent_id,
            text: options.text,
            sticker_id: match options.content {
                Content::Sticker { id } => id,
                _ => 0,
            },
            reply_to_id: options.reply_to_id.unwrap_or(0),
            answer_to_id: options.answer_to_id.unwrap_or(0),
            use_new_formatting: options.use_new_formatting,
            watch_post: options.watch_post,
            content: options.content,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    comment: RawPublication<RawComment>,
}

impl Request<Publication<Comment>> for CreateCommentRequest<'_> {
    type Response = Response;
    type Error = CreateCommentError;

    async fn send(&self, client: &Client) -> Result<Publication<Comment>> {
        let attachments: Vec<&[u8]> = match self.content {
            Content::Text | Content::Sticker { .. } => {
                vec![&[]]
            }
            Content::Images(ref images) => {
                let mut attachments: Vec<&[u8]> = vec![&[]];
                attachments.extend(images.iter());
                attachments
            }
            Content::Gif {
                first_frame,
                animated,
            } => vec![animated, first_frame],
        };

        client
            .send_request("RCommentsCreate", self, attachments)
            .await?
            .comment
            .try_into()
    }
}
