use serde::Deserialize;

use crate::Result;
use crate::models::ImageRef;
use crate::requests::Image;
use crate::requests::raw::RawImageRef;

pub(crate) struct Normal<'a> {
    image: &'a [u8],
}
impl<'a> Normal<'a> {
    pub(crate) fn new(image: &'a [u8]) -> Self {
        Self { image }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "image")]
    image: RawImageRef,
}

impl Image for Normal<'_> {
    type Target = ImageRef;
    type Response = Response;

    fn attachments(&self) -> Vec<&[u8]> {
        vec![self.image, &[]]
    }

    fn parse_response(&self, response: Response) -> Result<ImageRef> {
        Ok(response.image.into())
    }
}
