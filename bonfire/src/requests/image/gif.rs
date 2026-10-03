use serde::Deserialize;

use crate::Result;
use crate::models::ImageRef;
use crate::requests::Image;
use crate::requests::raw::RawImageRef;

pub(crate) struct Gif<'a> {
    first_frame: &'a [u8],
    animated: &'a [u8],
}
impl<'a> Gif<'a> {
    pub(crate) fn new(first_frame: &'a [u8], animated: &'a [u8]) -> Self {
        Self {
            first_frame,
            animated,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Response {
    #[serde(rename = "image")]
    first_frame: RawImageRef,
    #[serde(rename = "imageGif")]
    animated: RawImageRef,
}

impl Image for Gif<'_> {
    type Target = (ImageRef, ImageRef);
    type Response = Response;

    fn attachments(&self) -> Vec<&[u8]> {
        vec![self.first_frame, self.animated]
    }

    fn parse_response(&self, response: Response) -> Result<(ImageRef, ImageRef)> {
        Ok((response.first_frame.into(), response.animated.into()))
    }
}
